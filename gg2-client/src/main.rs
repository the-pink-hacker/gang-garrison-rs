use bevy::prelude::*;
use networking::NetworkingPlugin;

mod networking;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NetworkingPlugin)
        .run();
}
