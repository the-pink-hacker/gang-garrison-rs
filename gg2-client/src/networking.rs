use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use gg2_common::networking::message::*;
use socket::{AppNetworkClientMessage, ClientNetworkEvent, NetworkClient, NetworkSettings};

mod socket;
pub mod state;

pub use socket::NetworkData;
use state::NetworkingState;

const SERVER_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8150);

fn setup_networking(
    mut client: ResMut<NetworkClient>,
    network_settings: Res<NetworkSettings>,
    mut state: ResMut<NextState<NetworkingState>>,
) {
    client.connect(SERVER_ADDRESS, network_settings.clone());
    state.set(NetworkingState::AttemptingConnection);
}

fn on_network_event(
    client: ResMut<NetworkClient>,
    mut connection_events: EventReader<ClientNetworkEvent>,
    mut state: ResMut<NextState<NetworkingState>>,
) {
    for event in connection_events.read() {
        match event {
            ClientNetworkEvent::Connected => match client.send_message(ClientHello::default()) {
                Ok(_) => state.set(NetworkingState::AwaitingHello),
                Err(error) => eprintln!("Failed to send message: {}", error),
            },
            ClientNetworkEvent::Error(error) => {
                state.set(NetworkingState::Disconnected);
                eprintln!("Client network error: {}", error);
            }
            ClientNetworkEvent::Disconnected => {
                state.set(NetworkingState::Disconnected);
                println!("Disconnected from server.");
            }
        }
    }
}

fn handle_hello(
    mut hello_events: EventReader<NetworkData<ServerHello>>,
    client: ResMut<NetworkClient>,
    mut state: ResMut<NextState<NetworkingState>>,
) {
    for event in hello_events.read() {
        println!("{:#?}", **event);
        match client.send_message(ClientReserveSlot {
            player_name: "PlayerName".to_string(),
        }) {
            Ok(_) => state.set(NetworkingState::ReserveSlot),
            Err(error) => eprintln!("Failed to send message: {}", error),
        }
    }
}

fn handle_reserve_slot(
    mut reserve_events: EventReader<NetworkData<ServerReserveSlot>>,
    client: ResMut<NetworkClient>,
    mut state: ResMut<NextState<NetworkingState>>,
) {
    for _ in reserve_events.read() {
        println!("Joining server.");
        match client.send_message(ClientPlayerJoin) {
            Ok(_) => state.set(NetworkingState::PlayerJoining),
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn handle_server_full(
    mut server_full_events: EventReader<NetworkData<ServerServerFull>>,
    mut client: ResMut<NetworkClient>,
) {
    for _ in server_full_events.read() {
        println!("Server full.");
        client.disconnect();
    }
}

fn handle_join_update(mut join_update_events: EventReader<NetworkData<ServerJoinUpdate>>) {
    for event in join_update_events.read() {
        println!("{:#?}", **event);
    }
}

fn handle_change_map(mut change_map_events: EventReader<NetworkData<ServerChangeMap>>) {
    for event in change_map_events.read() {
        println!("{:#?}", **event);
    }
}

fn handle_full_update(mut events: EventReader<NetworkData<ServerFullUpdate>>) {
    for event in events.read() {
        println!("{:#?}", **event);
    }
}

fn handle_message_string(
    mut events: EventReader<NetworkData<ServerMessageString>>,
    mut state: ResMut<NextState<NetworkingState>>,
) {
    for event in events.read() {
        println!("{:#?}", **event);
        state.set(NetworkingState::InGame);
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .init_state::<NetworkingState>()
            .listen_for_client_message::<ServerHello>()
            .listen_for_client_message::<ServerReserveSlot>()
            .listen_for_client_message::<ServerServerFull>()
            .listen_for_client_message::<ServerInputState>()
            .listen_for_client_message::<ServerQuickUpdate>()
            .listen_for_client_message::<ServerPlayerJoin>()
            .listen_for_client_message::<ServerJoinUpdate>()
            .listen_for_client_message::<ServerChangeMap>()
            .listen_for_client_message::<ServerPlayerChangeClass>()
            .listen_for_client_message::<ServerPlayerChangeTeam>()
            .listen_for_client_message::<ServerFullUpdate>()
            .listen_for_client_message::<ServerMessageString>()
            .add_systems(
                FixedUpdate,
                (
                    on_network_event,
                    setup_networking.run_if(in_state(NetworkingState::Disconnected)),
                    handle_hello.run_if(in_state(NetworkingState::AwaitingHello)),
                    (handle_reserve_slot, handle_server_full)
                        .run_if(in_state(NetworkingState::ReserveSlot)),
                    (handle_join_update, handle_message_string)
                        .run_if(in_state(NetworkingState::PlayerJoining)),
                    (handle_change_map, handle_full_update)
                        .run_if(in_state(NetworkingState::PlayerJoining))
                        .run_if(in_state(NetworkingState::InGame)),
                ),
            );
    }
}
