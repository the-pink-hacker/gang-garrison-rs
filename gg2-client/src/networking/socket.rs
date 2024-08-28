// Loosly based on https://github.com/CabbitStudios/bevy_spicy_networking

use std::{net::SocketAddr, sync::Arc};

use bevy::{prelude::*, reflect::List};
use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use gg2_common::networking::{message::GGMessage, NetworkPacket, PacketKind};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("An error occured when accepting a new connnection: {0}")]
    Accept(std::io::Error),
    #[error("Could not find connection")]
    ConnectionNotFound,
    #[error("Connection closed")]
    ChannelClosed,
    #[error("Not connected to any server")]
    NotConnected,
    #[error("An error occured when trying to start listening for new connections: {0}")]
    Listen(std::io::Error),
    #[error("An error occured when trying to connect: {0}")]
    Connection(std::io::Error),
}

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

#[derive(Debug, Resource, Clone)]
pub struct NetworkSettings {
    pub max_packet_length: usize,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            // TODO: Find out good packet size
            // 10mb
            max_packet_length: 10 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Event)]
pub enum ClientNetworkEvent {
    Connected,
    Disconnected,
    Error(NetworkError),
}

#[derive(Debug)]
pub struct ServerConnection {
    peer_address: SocketAddr,
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

#[derive(Debug, Resource)]
pub struct NetworkClient {
    runtime: tokio::runtime::Runtime,
    server_connection: Option<ServerConnection>,
    receive_message_map: Arc<DashMap<u8, Vec<Vec<u8>>>>,
    network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr, NetworkSettings)>,
}

impl NetworkClient {
    pub fn connect(
        &mut self,
        address: impl ToSocketAddrs + Send + 'static,
        network_settings: NetworkSettings,
    ) {
        println!("Starting connection.");

        self.disconnect();

        let network_error_sender = self.network_events.sender.clone();
        let connection_event_sender = self.connection_events.sender.clone();

        self.runtime.spawn(async move {
            let stream = match TcpStream::connect(address).await {
                Ok(stream) => stream,
                Err(error) => {
                    if let Err(error) = network_error_sender
                        .send(ClientNetworkEvent::Error(NetworkError::Connection(error)))
                    {
                        println!("Couldn't send error event: {}", error);
                    };
                    return;
                }
            };

            let address = stream
                .peer_addr()
                .expect("Couldn't fetch peer_addr of existing stream");

            if let Err(error) = connection_event_sender.send((stream, address, network_settings)) {
                println!("Coudln't initiate connection: {}", error);
            }
        });
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

    pub fn send_message<T: GGMessage>(&self, message: T) -> Result<(), NetworkError> {
        println!("Sending message to server.");
        let server_connection = match self.server_connection.as_ref() {
            Some(server) => server,
            None => return Err(NetworkError::NotConnected),
        };

        if let Err(error) = server_connection.send_message.send(message.into()) {
            println!("Server disconnected: {}", error);
            return Err(NetworkError::NotConnected);
        }

        Ok(())
    }
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Couldn't build tokio runtime"),
            server_connection: None,
            receive_message_map: Arc::default(),
            network_events: SyncChannel::default(),
            connection_events: SyncChannel::default(),
        }
    }
}

pub fn handle_connection_event(
    mut client: ResMut<NetworkClient>,
    mut events: EventWriter<ClientNetworkEvent>,
) {
    let (connection, peer_address, network_settings) =
        match client.connection_events.receiver.try_recv() {
            Ok(event) => event,
            Err(_err) => {
                return;
            }
        };

    let (read_socket, send_socket) = connection.into_split();
    let (send_message, receive_message) = unbounded_channel();

    client.server_connection = Some(ServerConnection {
        peer_address,
        send_task: client.runtime.spawn(send_task(
            receive_message,
            send_socket,
            client.network_events.sender.clone(),
        )),
        receive_task: client.runtime.spawn(receive_task(
            read_socket,
            network_settings,
            client.receive_message_map.clone(),
            peer_address,
            client.network_events.sender.clone(),
        )),
        send_message,
    });

    events.send(ClientNetworkEvent::Connected);
}

async fn send_task(
    mut receive_message: UnboundedReceiver<NetworkPacket>,
    mut send_socket: OwnedWriteHalf,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    println!("Starting new server connection; sending task.");

    while let Some(message) = receive_message.recv().await {
        let message_kind = message.kind;
        let encoded_message = Vec::from(message);

        if let Err(error) = send_socket.write_all(&encoded_message).await {
            println!("Couldn't send packet: {:?}: {}", message_kind, error);
        }

        println!("Succesfully written all!");
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

async fn receive_task(
    mut read_socket: OwnedReadHalf,
    network_settings: NetworkSettings,
    receive_message_map: Arc<DashMap<u8, Vec<Vec<u8>>>>,
    peer_address: SocketAddr,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    // TODO: Actually implement
    //let mut buffer = (0..network_settings.max_packet_length)
    //    .map(|_| 0)
    //    .collect::<Vec<u8>>();
    //loop {
    //    let length = match read_socket.read_u32().await {
    //        Ok(len) => len as usize,
    //        Err(err) => {
    //            error!(
    //                "Encountered error while fetching length [{}]: {}",
    //                peer_address, err
    //            );
    //            break;
    //        }
    //    };
    //
    //    if length > network_settings.max_packet_length {
    //        error!(
    //            "Received too large packet from [{}]: {} > {}",
    //            peer_address, length, network_settings.max_packet_length
    //        );
    //        break;
    //    }
    //
    //    match read_socket.read_exact(&mut buffer[..length]).await {
    //        Ok(_) => (),
    //        Err(err) => {
    //            error!(
    //                "Encountered error while fetching stream of length {} [{}]: {}",
    //                length, peer_address, err
    //            );
    //            break;
    //        }
    //    }
    //
    //    let packet: NetworkPacket = match bincode::deserialize(&buffer[..length]) {
    //        Ok(packet) => packet,
    //        Err(err) => {
    //            error!(
    //                "Failed to decode network packet from [{}]: {}",
    //                peer_addr, err
    //            );
    //            break;
    //        }
    //    };
    //
    //    match receive_message_map.get_mut(&packet.kind.into()) {
    //        Some(mut packets) => packets.push(packet.data),
    //        None => {
    //            error!(
    //                "Could not find existing entries for message kinds: {:?}",
    //                packet
    //            );
    //        }
    //    }
    //    debug!("Received message from: {}", peer_addr);
    //}
    //
    //let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

pub fn send_client_network_events(
    client_server: ResMut<NetworkClient>,
    mut client_network_events: EventWriter<ClientNetworkEvent>,
) {
    client_network_events.send_batch(client_server.network_events.receiver.try_iter());
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkClient::default())
            .add_event::<ClientNetworkEvent>()
            .init_resource::<NetworkSettings>()
            .add_systems(
                PreUpdate,
                (send_client_network_events, handle_connection_event),
            );
    }
}
