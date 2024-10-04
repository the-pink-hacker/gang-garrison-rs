use bevy::prelude::*;

mod camera;
mod config;
mod game;
mod gui;
mod map;
mod networking;
mod physics;
mod player;
mod state;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gang Garrison 2: Rust".to_string(),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
            camera::CameraPlugin,
            config::ClientConfigPlugin,
            gui::GuiPlugin,
            map::MapPlugin,
            networking::NetworkingPlugin,
            physics::ClientPhysicsPlugin,
            player::PlayerPlugin,
            state::ClientStatePlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
