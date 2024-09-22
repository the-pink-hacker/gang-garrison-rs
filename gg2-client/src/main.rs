use bevy::prelude::*;

mod camera;
mod map;
mod networking;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            networking::NetworkingPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            map::MapPlugin,
            physics::ClientPhysicsPlugin,
        ))
        .run();
}
