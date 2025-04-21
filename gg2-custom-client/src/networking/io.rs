use std::{net::SocketAddr, sync::Arc};

use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use gg2_client::networking::message::{ClientNetworkDeserialize, ClientNetworkSerialize};
use gg2_common::networking::{NetworkPacket, PacketKind, error::*, message::GGMessage};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    task::JoinHandle,
};

use crate::prelude::{UpdateMutRunnable, World, debug, error, info, trace};

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
    Error(Error),
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
    tried_connect: bool, // TODO: REMOVE; IS FOR DEBUG
    server_connection: Option<ServerConnection>,
    receive_message_map: Arc<DashMap<PacketKind, Vec<Vec<u8>>>>,
    network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr)>,
}

impl NetworkClient {
    // Connects to a new server
    pub async fn connect(&mut self, address: SocketAddr) {
        debug!("Starting connection.");

        if self.server_connection.is_some() {
            self.disconnect();
        }

        let network_error_sender = self.network_events.sender.clone();
        let connection_event_sender = self.connection_events.sender.clone();

        let stream = match TcpStream::connect(address).await {
            Ok(stream) => stream,
            Err(error) => {
                if let Err(error) = network_error_sender
                    .send(ClientNetworkEvent::Error(Error::Connection(error, address)))
                {
                    error!("Couldn't send error event: {}", error);
                };
                return;
            }
        };

        let address = stream
            .peer_addr()
            .expect("Couldn't fetch peer_addr of existing stream");

        if let Err(error) = connection_event_sender.send((stream, address)) {
            error!("Coudln't initiate connection: {}", error);
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(connection) = self.server_connection.take() {
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
            .ok_or(Error::NotConnected)?
            .send_message
            .send(NetworkPacket::from_message(message)?)
            .map_err(|_| Error::NotConnected)
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
                    self.receive_message_map.clone(),
                    peer_address,
                    self.network_events.sender.clone(),
                )),
                send_message,
            });

            info!("Connected to server {}", peer_address);
        };
    }

    fn handle_network_events(&mut self) {
        self.network_events
            .receiver
            .try_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|event| match event {
                ClientNetworkEvent::Connected => debug!("Network Event: Connected to server"),
                ClientNetworkEvent::Disconnected => {
                    debug!("Network Event: Disconnected from server");
                    self.tried_connect = false;
                }
                ClientNetworkEvent::Error(error) => error!("Network Event: {}", error),
            });
    }
}

impl UpdateMutRunnable for NetworkClient {
    async fn update_mut(&mut self, world: &World) {
        self.handle_connection_event();
        self.handle_network_events();

        if self.is_connected() {
            info!("Connected");
        } else if !self.tried_connect {
            info!("Trying connection");
            self.connect(std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
                std::net::Ipv4Addr::LOCALHOST,
                crate::networking::io::DEFAULT_PORT,
            )))
            .await;
            self.tried_connect = true;
        }
    }
}

// Sends network packets to server
async fn send_task(
    mut receive_message: UnboundedReceiver<NetworkPacket>,
    mut send_socket: OwnedWriteHalf,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    debug!("Starting new server connection; sending task.");

    while let Some(message) = receive_message.recv().await {
        let message_kind = message.kind;
        let encoded_message = Vec::from(message);
        trace!("Sending: {}", encoded_message.escape_ascii());

        if let Err(error) = send_socket.write_all(&encoded_message).await {
            error!("Couldn't send packet: {:?}: {}", message_kind, error);
        }

        trace!("Succesfully written all!");
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

// Receives data from server and passes network packets
async fn receive_task(
    mut read_socket: OwnedReadHalf,
    receive_message_map: Arc<DashMap<PacketKind, Vec<Vec<u8>>>>,
    peer_address: SocketAddr,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    let mut buffer = vec![0; MAX_PACKET_LENGTH];
    loop {
        let length = read_socket.read(&mut buffer).await.unwrap();
        trace!(
            "Received {} bytes: {}",
            length,
            buffer[0..length].escape_ascii()
        );

        let packet: NetworkPacket = match NetworkPacket::try_from(&buffer[..length]) {
            Ok(packet) => packet,
            Err(error) => {
                error!(
                    "Failed to decode network packet from [{}]: {}",
                    peer_address, error
                );
                break;
            }
        };

        let packet_kind = packet.kind;
        trace!("Packet kind: {:?}", packet_kind);

        match receive_message_map.get_mut(&packet_kind) {
            Some(mut packets) => packets.push(packet.data),
            None => {
                error!(
                    "Couldn't find existing entries for message kinds: {:?}",
                    packet_kind
                );
            }
        }
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}
