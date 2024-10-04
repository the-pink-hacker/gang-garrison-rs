use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::state::InGamePauseState;

fn pause_game(mut state: ResMut<NextState<InGamePauseState>>) {
    state.set(InGamePauseState::Paused);
}

fn unpause_game(mut state: ResMut<NextState<InGamePauseState>>) {
    state.set(InGamePauseState::None);
}

pub struct GuiInputPlugin;

impl Plugin for GuiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pause_game.run_if(in_state(InGamePauseState::None)),
                unpause_game.run_if(in_state(InGamePauseState::Paused)),
            )
                .run_if(input_just_pressed(KeyCode::Escape)),
        );
    }
}
