use bevy::prelude::*;
use gg2_common::game::*;

use crate::state::ClientState;

pub struct ClientGamePlugin;

impl Plugin for ClientGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(ClientState::InGame),
            garbage_collect_in_game_only_system,
        );
    }
}
