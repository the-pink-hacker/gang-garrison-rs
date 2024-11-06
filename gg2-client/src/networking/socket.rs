// Loosly based on https://github.com/CabbitStudios/bevy_spicy_networking

use std::{net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use gg2_common::networking::{
    error::{Error, Result},
    message::{GGMessage, NetworkDeserialize, NetworkSerialize},
    NetworkPacket, PacketKind,
};
use log::debug;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

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
            max_packet_length: 1024,
        }
    }
}

#[derive(Debug, Event)]
pub enum ClientNetworkEvent {
    Connected,
    Disconnected,
    Error(gg2_common::networking::error::Error),
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

#[derive(Debug, Resource)]
pub struct NetworkClient {
    runtime: tokio::runtime::Runtime,
    server_connection: Option<ServerConnection>,
    receive_message_map: Arc<DashMap<PacketKind, Vec<Vec<u8>>>>,
    network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr, NetworkSettings)>,
}

impl NetworkClient {
    // Connects to a new server
    pub fn connect(&mut self, address: SocketAddr, network_settings: NetworkSettings) {
        debug!("Starting connection.");

        if self.server_connection.is_some() {
            self.disconnect();
        }

        let network_error_sender = self.network_events.sender.clone();
        let connection_event_sender = self.connection_events.sender.clone();

        self.runtime.spawn(async move {
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

            if let Err(error) = connection_event_sender.send((stream, address, network_settings)) {
                error!("Coudln't initiate connection: {}", error);
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

    pub fn send_message<T: NetworkSerialize + GGMessage>(&self, message: T) -> Result<()> {
        trace!("Sending message to server.");
        self.server_connection
            .as_ref()
            .ok_or(Error::NotConnected)?
            .send_message
            .send(NetworkPacket::from_message(message)?)
            .map_err(|_| Error::NotConnected)
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

// Sets up send and receive threads
pub fn handle_connection_event_system(
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
        debug!("Sending: {}", encoded_message.escape_ascii());

        if let Err(error) = send_socket.write_all(&encoded_message).await {
            error!("Couldn't send packet: {:?}: {}", message_kind, error);
        }

        debug!("Succesfully written all!");
    }

    let _ = network_event_sender.send(ClientNetworkEvent::Disconnected);
}

// Receives data from server and passes network packets
async fn receive_task(
    mut read_socket: OwnedReadHalf,
    network_settings: NetworkSettings,
    receive_message_map: Arc<DashMap<PacketKind, Vec<Vec<u8>>>>,
    peer_address: SocketAddr,
    network_event_sender: Sender<ClientNetworkEvent>,
) {
    let mut buffer = vec![0; network_settings.max_packet_length];
    loop {
        let length = read_socket.read(&mut buffer).await.unwrap();
        debug!(
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
        debug!("Packet kind: {:?}", packet_kind);

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

pub fn send_client_network_events_system(
    client_server: ResMut<NetworkClient>,
    mut client_network_events: EventWriter<ClientNetworkEvent>,
) {
    client_network_events.send_batch(client_server.network_events.receiver.try_iter());
}

#[derive(Debug, Deref, Event)]
pub struct NetworkData<T: Send + Sync> {
    #[deref]
    inner: T,
}

pub trait AppNetworkClientMessage {
    fn listen_for_client_message<T: NetworkDeserialize + GGMessage + 'static>(
        &mut self,
    ) -> &mut Self;
}

impl AppNetworkClientMessage for App {
    // Registers message events for the client
    fn listen_for_client_message<T: NetworkDeserialize + GGMessage + 'static>(
        &mut self,
    ) -> &mut Self {
        let client = self
            .world()
            .get_resource::<NetworkClient>()
            .expect("Failed to get network client");

        assert!(
            !client.receive_message_map.contains_key(&T::KIND),
            "Duplicate registration of client message: {:?}",
            T::KIND
        );

        debug!("Register a new client message: {:?}", T::KIND);

        client.receive_message_map.insert(T::KIND, Vec::new());

        self.add_event::<NetworkData<T>>()
            .add_systems(FixedPreUpdate, register_client_message_system::<T>)
    }
}

// Reads in network packets and passes messages to Bevy
fn register_client_message_system<T: NetworkDeserialize + GGMessage + 'static>(
    client: ResMut<NetworkClient>,
    mut events: EventWriter<NetworkData<T>>,
) {
    let mut messages = match client.receive_message_map.get_mut(&T::KIND) {
        Some(message) => message,
        None => return,
    };

    events.send_batch(
        messages
            .drain(..)
            .map(|bytes| {
                let mut bytes = bytes.into_iter();
                let output = T::deserialize(&mut bytes);
                if output.is_ok() && bytes.len() > 0 {
                    if let Some(kind) = bytes.next().and_then(|raw| PacketKind::try_from(raw).ok())
                    {
                        debug!("Another packet was found of type: {:?}", kind);
                        match &mut client.receive_message_map.get_mut(&kind) {
                            Some(messages) => messages.push(bytes.collect()),
                            None => error!(
                                "Couldn't find existing entries for message kinds: {:?}",
                                kind
                            ),
                        }
                    }
                }
                output
            })
            .filter_map(|message| match message {
                Ok(message) => Some(message),
                Err(error) => {
                    error!("Failed to deserialize message: {}", error);
                    None
                }
            })
            .map(|msg| NetworkData { inner: msg }),
    );
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkClient::default())
            .add_event::<ClientNetworkEvent>()
            .init_resource::<NetworkSettings>()
            .add_systems(
                FixedPreUpdate,
                (
                    send_client_network_events_system,
                    handle_connection_event_system,
                ),
            );
    }
}
