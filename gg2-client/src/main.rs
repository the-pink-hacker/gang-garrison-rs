use bevy::prelude::*;

mod camera;
mod networking;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(networking::NetworkingPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .run();
}
