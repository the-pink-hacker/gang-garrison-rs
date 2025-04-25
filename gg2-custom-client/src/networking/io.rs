use std::{
    collections::VecDeque,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crossbeam_channel::{Receiver, Sender};
use gg2_client::networking::{
    message::{ClientNetworkSerialize, server::ServerMessageGeneric},
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
    send_message: UnboundedSender<Vec<u8>>,
}

impl ServerConnection {
    fn stop(self) {
        self.receive_task.abort();
        self.send_task.abort();
    }
}

#[derive(Debug, Default)]
pub struct NetworkClient {
    server_connection: Option<ServerConnection>,
    receive_message: Arc<Mutex<VecDequeIter<u8>>>,
    pub network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr)>,
    pub connection_state: NetworkingState,
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

    fn send_raw(&self, buffer: Vec<u8>) -> Result<()> {
        trace!("Sending message to server.");
        self.server_connection
            .as_ref()
            .ok_or(NetworkError::NotConnected)?
            .send_message
            .send(buffer)
            .map_err(|_| Error::NetworkError(NetworkError::NotConnected))
    }

    pub fn send<T: ClientNetworkSerialize>(&self, message: T) -> Result<()> {
        let mut buffer = Vec::with_capacity(256);
        message.serialize(&mut buffer)?;

        self.send_raw(buffer)
    }

    pub fn send_message<T: ClientNetworkSerialize + GGMessage>(&self, message: T) -> Result<()> {
        let mut buffer = Vec::with_capacity(256);
        buffer.push(T::KIND.into());
        message.serialize(&mut buffer)?;

        self.send_raw(buffer)
    }

    /// Sets up send and receive threads when connecting
    pub fn handle_connection_event(&mut self) {
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

    pub async fn pop_message(&mut self) -> Result<Option<ServerMessageGeneric>> {
        let recieve_message = &mut *self.receive_message.lock().await;

        if recieve_message.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ServerMessageGeneric::take(recieve_message)?))
        }
    }
}

// Sends network packets to server
async fn send_task(
    mut receive_message: UnboundedReceiver<Vec<u8>>,
    mut send_socket: OwnedWriteHalf,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    while let Some(message) = receive_message.recv().await {
        trace!("Sending: {}", message.escape_ascii());

        if let Err(error) = send_socket.write_all(&message).await {
            let _ = network_event_sender.send(ClientNetworkEvent::Error(NetworkError::PacketSend));
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
