use bevy::prelude::*;

mod socket;

fn setup_networking() {
    print!("hi, {}", gg2_common::networking::PacketId::Hello as u8);
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .add_systems(Startup, setup_networking);
    }
}
