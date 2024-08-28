// Loosly based on https://github.com/CabbitStudios/bevy_spicy_networking

use std::{net::SocketAddr, sync::Arc};

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    runtime::Runtime,
    sync::mpsc::{unbounded_channel, UnboundedSender},
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

#[derive(Debug, Resource)]
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
    Error,
}

#[derive(Debug, Resource)]
pub struct NetworkClient {
    runtime: tokio::runtime::Runtime,
    server_connection: Option<()>,
    receive_message_map: Arc<DashMap<&'static str, Vec<Vec<u8>>>>,
    network_events: SyncChannel<ClientNetworkEvent>,
    connection_events: SyncChannel<(TcpStream, SocketAddr, NetworkSettings)>,
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
    mut network_resource: ResMut<NetworkClient>,
    mut events: EventWriter<ClientNetworkEvent>,
) {
}

pub fn send_client_network_events(
    client_server: ResMut<NetworkClient>,
    mut client_network_events: EventWriter<ClientNetworkEvent>,
) {
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
