use bevy::prelude::*;

mod socket;

fn setup_networking() {
    print!("hi");
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(socket::ClientPlugin)
            .add_systems(Startup, setup_networking);
    }
}
