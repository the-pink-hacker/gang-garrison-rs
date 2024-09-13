use bevy::prelude::*;

mod camera;
mod map;
mod networking;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            //networking::NetworkingPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            map::MapPlugin,
        ))
        .run();
}
