use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use gg2_common::networking::message::{ClientHello, ServerHello};
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
                if let Err(error) = client.send_message(ClientHello::default()) {
                    println!("Failed to send message: {}", error);
                }
            }
            ClientNetworkEvent::Error(error) => {
                println!("Client network error: {}", error);
            }
            ClientNetworkEvent::Disconnected => {
                println!("Disconnected from server.");
            }
        }
    }
}

fn hello_server(mut hello_events: EventReader<NetworkData<ServerHello>>) {
    for hello_event in hello_events.read() {
        println!("{:#?}", hello_event);
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin);

        app.listen_for_client_message::<ServerHello>();

        app.add_systems(Startup, setup_networking)
            .add_systems(FixedUpdate, (on_network_event, hello_server));
    }
}
