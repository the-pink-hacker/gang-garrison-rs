use bevy::prelude::*;

fn test() {
    println!("Hello, world!");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, test)
        .run();
}
