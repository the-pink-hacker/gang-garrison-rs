use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::prelude::*;
use socket::{NetworkClient, NetworkSettings};

mod socket;

const SERVER_ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8150);

fn setup_networking(mut client: ResMut<NetworkClient>, network_settings: Res<NetworkSettings>) {
    client.connect(SERVER_ADDRESS, network_settings.clone());
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .add_systems(Startup, setup_networking);
    }
}
