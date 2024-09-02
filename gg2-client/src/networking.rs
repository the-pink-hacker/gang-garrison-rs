use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use gg2_common::{
    networking::message::{
        ClientHello, ClientPlayerJoin, ClientReserveSlot, ServerChangeMap, ServerHello,
        ServerInputState, ServerJoinUpdate, ServerPlayerChangeClass, ServerPlayerJoin,
        ServerQuickUpdate, ServerReserveSlot, ServerServerFull,
    },
    player::Player,
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
            .send_message(ClientPlayerJoin)
            .inspect_err(|error| eprintln!("{}", error));
    }
}

fn server_full(mut server_full_events: EventReader<NetworkData<ServerServerFull>>) {
    for _ in server_full_events.read() {
        println!("Server full.");
    }
}

fn player_join(
    mut player_join_events: EventReader<NetworkData<ServerPlayerJoin>>,
    mut commands: Commands,
) {
    for event in player_join_events.read() {
        println!("{:#?}", event);
        commands.spawn(Player {
            name: event.player_name.clone(),
        });
    }
}

fn join_update(mut join_update_events: EventReader<NetworkData<ServerJoinUpdate>>) {
    for event in join_update_events.read() {
        println!("{:#?}", event);
    }
}

fn change_map(mut change_map_events: EventReader<NetworkData<ServerChangeMap>>) {
    for event in change_map_events.read() {
        println!("{:#?}", event);
    }
}

fn player_change_class(
    mut player_change_class_events: EventReader<NetworkData<ServerPlayerChangeClass>>,
) {
    for event in player_change_class_events.read() {
        println!("{:#?}", event);
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .listen_for_client_message::<ServerHello>()
            .listen_for_client_message::<ServerReserveSlot>()
            .listen_for_client_message::<ServerServerFull>()
            .listen_for_client_message::<ServerInputState>()
            .listen_for_client_message::<ServerQuickUpdate>()
            .listen_for_client_message::<ServerPlayerJoin>()
            .listen_for_client_message::<ServerJoinUpdate>()
            .listen_for_client_message::<ServerChangeMap>()
            .listen_for_client_message::<ServerPlayerChangeClass>()
            .add_systems(Startup, setup_networking)
            .add_systems(
                FixedUpdate,
                (
                    on_network_event,
                    hello_server,
                    reserve_slot,
                    server_full,
                    player_join,
                    join_update,
                    change_map,
                    player_change_class,
                ),
            );
    }
}
