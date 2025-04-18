use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    config::ClientConfig,
    state::{InGameDebugState, InGamePauseState},
};

fn pause_game_system(mut state: ResMut<NextState<InGamePauseState>>) {
    state.set(InGamePauseState::Paused);
}

fn unpause_game_system(mut state: ResMut<NextState<InGamePauseState>>) {
    state.set(InGamePauseState::None);
}

fn enable_debug_state_system(
    mut state: ResMut<NextState<InGameDebugState>>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<ClientConfig>,
) {
    if input.just_pressed(config.controls.debug_menu) {
        state.set(InGameDebugState::Enabled);
    }
}

fn disable_debug_state_system(
    mut state: ResMut<NextState<InGameDebugState>>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<ClientConfig>,
) {
    if input.just_pressed(config.controls.debug_menu) {
        state.set(InGameDebugState::Disabled);
    }
}

pub struct GuiInputPlugin;

impl Plugin for GuiInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    pause_game_system.run_if(in_state(InGamePauseState::None)),
                    unpause_game_system.run_if(in_state(InGamePauseState::Paused)),
                )
                    .run_if(input_just_pressed(KeyCode::Escape)),
                enable_debug_state_system.run_if(in_state(InGameDebugState::Disabled)),
                disable_debug_state_system.run_if(in_state(InGameDebugState::Enabled)),
            ),
        );
    }
}
