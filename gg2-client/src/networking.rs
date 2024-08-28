use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use gg2_common::networking::message::GGMessageHello;
use socket::{NetworkClient, NetworkSettings};

mod socket;

const SERVER_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8150);

fn setup_networking(mut client: ResMut<NetworkClient>, network_settings: Res<NetworkSettings>) {
    client.connect(SERVER_ADDRESS, network_settings.clone());
}

fn hello(client: ResMut<NetworkClient>) {
    if let Err(error) = client.send_message(GGMessageHello) {
        println!("Failed to send message: {}", error);
    }
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .add_systems(Startup, setup_networking)
            .add_systems(Update, hello);
    }
}
