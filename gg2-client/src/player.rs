use bevy::prelude::*;
use gg2_common::player::Player;

fn update_player_position(player_query: Query<&Player>) {
    for player in player_query.iter() {
        println!("Player of name: {}", player.name);
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_player_position);
    }
}
