use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::state::InGameVisualState;

fn pause_game(mut state: ResMut<NextState<InGameVisualState>>) {
    state.set(InGameVisualState::Paused);
}

fn unpause_game(mut state: ResMut<NextState<InGameVisualState>>) {
    state.set(InGameVisualState::None);
}

pub struct GuiInputPlugin;

impl Plugin for GuiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pause_game.run_if(in_state(InGameVisualState::None)),
                unpause_game.run_if(in_state(InGameVisualState::Paused)),
            )
                .run_if(input_just_pressed(KeyCode::Escape)),
        );
    }
}
