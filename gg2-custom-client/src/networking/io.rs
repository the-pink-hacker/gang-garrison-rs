use std::{
    collections::VecDeque,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crossbeam_channel::{Receiver, Sender};
use gg2_client::networking::{
    message::{ClientNetworkDeserialize, ClientNetworkSerialize, server::ServerMessageGeneric},
    state::NetworkingState,
};
use gg2_common::networking::{NetworkPacket, error::Error as NetworkError, message::*};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        Mutex,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    task::JoinHandle,
};

use crate::prelude::*;

pub const MAX_PACKET_LENGTH: usize = 1024;
pub const DEFAULT_PORT: u16 = 8190;

#[derive(Debug)]
pub struct SyncChannel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> Default for SyncChannel<T> {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self { sender, receiver }
    }
}

#[derive(Debug)]
pub enum ClientNetworkEvent {
    Connected,
    Disconnected,
    Error(NetworkError),
}

#[derive(Debug)]
pub struct ServerConnection {
    receive_task: JoinHandle<()>,
    send_task: JoinHandle<()>,
    send_message: UnboundedSender<NetworkPacket>,
}

impl ServerConnection {
    fn stop(self) {
        self.receive_task.abort();
        self.send_task.abort();
    }
}

trait FromMessage {
    fn from_message<T: GGMessage + ClientNetworkSerialize>(message: T) -> Result<Self>
    where
        Self: Sized;
}

impl FromMessage for NetworkPacket {
    fn from_message<T: GGMessage + ClientNetworkSerialize>(message: T) -> Result<Self> {
        let mut data = Vec::new();
        message.serialize(&mut data)?;

        Ok(Self {
            kind: T::KIND,
            data,
        })
    }
}

#[derive(Debug, Default)]
pub struct NetworkClient {
    server_connection: Option<ServerConnection>,
    receive_message: Arc<Mutex<VecDequeIter<u8>>>,
    network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr)>,
    connection_state: NetworkingState,
}

impl NetworkClient {
    // Connects to a new server
    pub async fn connect(&mut self, address: SocketAddr) -> Result<()> {
        info!("Connecting to server: {}", address);

        if self.server_connection.is_some() {
            self.disconnect();
        }

        let network_error_sender = self.network_events.sender.clone();
        let connection_event_sender = self.connection_events.sender.clone();

        let stream = match TcpStream::connect(address).await {
            Ok(stream) => stream,
            Err(error) => {
                return Err(Error::NetworkError(NetworkError::Connection(
                    error, address,
                )));
            }
        };

        let address = stream.peer_addr().map_err(|_| NetworkError::NotConnected)?;

        connection_event_sender
            .send((stream, address))
            .map_err(|_| Error::NetworkError(NetworkError::ConnectSend))
    }

    pub fn disconnect(&mut self) {
        if let Some(connection) = self.server_connection.take() {
            self.connection_state = NetworkingState::Disconnected;
            connection.stop();

            let _ = self
                .network_events
                .sender
                .send(ClientNetworkEvent::Disconnected);
        }
    }

    pub fn send_message<T: ClientNetworkSerialize + GGMessage>(&self, message: T) -> Result<()> {
        trace!("Sending message to server.");
        self.server_connection
            .as_ref()
            .ok_or(NetworkError::NotConnected)?
            .send_message
            .send(NetworkPacket::from_message(message)?)
            .map_err(|_| Error::NetworkError(NetworkError::NotConnected))
    }

    pub fn is_connected(&self) -> bool {
        self.server_connection.is_some()
    }

    /// Sets up send and receive threads when connecting
    fn handle_connection_event(&mut self) {
        if let Ok((connection, peer_address)) = self.connection_events.receiver.try_recv() {
            let (read_socket, send_socket) = connection.into_split();
            let (send_message, receive_message) = unbounded_channel();

            self.server_connection = Some(ServerConnection {
                send_task: tokio::spawn(send_task(
                    receive_message,
                    send_socket,
                    self.network_events.sender.clone(),
                )),
                receive_task: tokio::spawn(receive_task(
                    read_socket,
                    Arc::clone(&self.receive_message),
                    peer_address,
                    self.network_events.sender.clone(),
                )),
                send_message,
            });

            let _ = self
                .network_events
                .sender
                .send(ClientNetworkEvent::Connected);
        };
    }

    fn handle_network_events(&mut self) -> Result<()> {
        let events = self.network_events.receiver.try_iter().collect::<Vec<_>>();

        for event in events {
            match event {
                ClientNetworkEvent::Connected => {
                    debug!("Network Event: Connected to server");
                    info!("Connected to server; sending hello");
                    self.send_message(ClientHello::default())?;
                    self.connection_state = NetworkingState::AwaitingHello;
                }
                ClientNetworkEvent::Disconnected => {
                    debug!("Network Event: Disconnected from server");
                    self.disconnect();
                }
                ClientNetworkEvent::Error(error) => Err(error)?,
            }
        }

        Ok(())
    }

    pub async fn pop_message(&mut self) -> Result<Option<ServerMessageGeneric>> {
        let recieve_message = &mut *self.receive_message.lock().await;

        if recieve_message.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ServerMessageGeneric::take(recieve_message)?))
        }
    }
}

impl UpdateMutRunnable for NetworkClient {
    async fn update_mut(&mut self, world: &World) -> Result<()> {
        self.handle_connection_event();
        self.handle_network_events()?;

        match self.connection_state {
            NetworkingState::Disconnected => {
                self.connect(std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::LOCALHOST,
                    DEFAULT_PORT,
                )))
                .await?;
                self.connection_state = NetworkingState::AttemptingConnection;
            }
            // Handled in `Self::handle_network_events`
            NetworkingState::AttemptingConnection => (),
            NetworkingState::AwaitingHello => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::Hello(message) => {
                            debug!("{:#?}", message);
                            self.send_message(ClientReserveSlot {
                                player_name: "Rust Client".to_string(),
                            })?;
                            self.connection_state = NetworkingState::ReserveSlot;
                        }
                        // TODO: ServerMessageGeneric::PasswordRequest => (),
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::ReserveSlot => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::ServerFull(message) => {
                            info!("Server full");
                            self.disconnect();
                        }
                        ServerMessageGeneric::ReserveSlot(message) => {
                            debug!("{:#?}", message);
                            self.connection_state = NetworkingState::PlayerJoining;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::PlayerJoining => (),
            NetworkingState::InGame => (),
        }

        Ok(())
    }
}

// Sends network packets to server
async fn send_task(
    mut receive_message: UnboundedReceiver<NetworkPacket>,
    mut send_socket: OwnedWriteHalf,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    while let Some(message) = receive_message.recv().await {
        let message_kind = message.kind;
        let encoded_message = Vec::from(message);
        trace!("Sending: {}", encoded_message.escape_ascii());

        if let Err(error) = send_socket.write_all(&encoded_message).await {
            let _ = network_event_sender.send(ClientNetworkEvent::Error(NetworkError::PacketSend(
                message_kind,
            )));
        }

        trace!("Succesfully written all!");
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

// Receives data from server and passes network packets
async fn receive_task(
    mut read_socket: OwnedReadHalf,
    receive_messages: Arc<Mutex<VecDequeIter<u8>>>,
    peer_address: SocketAddr,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    let mut buffer = [0; MAX_PACKET_LENGTH];
    loop {
        let length = read_socket.read(&mut buffer).await.unwrap();
        trace!(
            "Received {} bytes: {}",
            length,
            buffer[0..length].escape_ascii()
        );

        receive_messages.lock().await.extend(&buffer[..length]);
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

#[derive(Debug, Default)]
struct VecDequeIter<T>(VecDeque<T>);

impl<T> Iterator for VecDequeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> Deref for VecDequeIter<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for VecDequeIter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_deque_iter() {
        let mut x = super::VecDequeIter(vec![0, 1, 2, 3, 4].into());

        assert_eq!(x.next(), Some(0));
        assert_eq!(x.next(), Some(1));

        x.extend([100, 200]);

        assert_eq!(x.next(), Some(2));
        assert_eq!(x.next(), Some(3));
        assert_eq!(x.next(), Some(4));
        assert_eq!(x.next(), Some(100));
        assert_eq!(x.next(), Some(200));
        assert_eq!(x.next(), None);
    }
}
