use bevy::prelude::*;

mod config;
mod game;
mod map;
mod networking;
mod physics;
mod player;
mod render;
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
            config::ClientConfigPlugin,
            map::MapPlugin,
            networking::NetworkingPlugin,
            physics::ClientPhysicsPlugin,
            player::PlayerPlugin,
            state::ClientStatePlugin,
            game::ClientGamePlugin,
            render::RenderPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
