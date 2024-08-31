use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use gg2_common::networking::message::{
    ClientHello, ClientPlayerJoin, ClientReserveSlot, ServerHello, ServerReserveSlot,
    ServerServerFull,
};
use socket::{
    AppNetworkClientMessage, ClientNetworkEvent, NetworkClient, NetworkData, NetworkSettings,
};

mod socket;

const SERVER_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8150);

fn setup_networking(mut client: ResMut<NetworkClient>, network_settings: Res<NetworkSettings>) {
    client.connect(SERVER_ADDRESS, network_settings.clone());
}

fn on_network_event(
    client: ResMut<NetworkClient>,
    mut connection_events: EventReader<ClientNetworkEvent>,
) {
    for event in connection_events.read() {
        match event {
            ClientNetworkEvent::Connected => {
                let _ = client
                    .send_message(ClientHello::default())
                    .inspect_err(|error| println!("Failed to send message: {}", error));
            }
            ClientNetworkEvent::Error(error) => {
                eprintln!("Client network error: {}", error);
            }
            ClientNetworkEvent::Disconnected => {
                println!("Disconnected from server.");
            }
        }
    }
}

fn hello_server(
    mut hello_events: EventReader<NetworkData<ServerHello>>,
    client: ResMut<NetworkClient>,
) {
    for event in hello_events.read() {
        println!("{:#?}", **event);
        let _ = client
            .send_message(ClientReserveSlot {
                player_name: "PlayerName".to_string(),
            })
            .inspect_err(|error| eprintln!("Failed to send message: {}", error));
    }
}

fn reserve_slot(
    mut reserve_events: EventReader<NetworkData<ServerReserveSlot>>,
    client: ResMut<NetworkClient>,
) {
    for _ in reserve_events.read() {
        println!("Joining server.");
        let _ = client
            .send_message(ClientPlayerJoin {})
            .inspect_err(|error| eprintln!("{}", error));
    }
}

fn server_full(mut server_full_events: EventReader<NetworkData<ServerServerFull>>) {
    for _ in server_full_events.read() {
        println!("Server full.");
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin);

        app.listen_for_client_message::<ServerHello>();
        app.listen_for_client_message::<ServerReserveSlot>();
        app.listen_for_client_message::<ServerServerFull>();

        app.add_systems(Startup, setup_networking).add_systems(
            FixedUpdate,
            (on_network_event, hello_server, reserve_slot, server_full),
        );
    }
}
