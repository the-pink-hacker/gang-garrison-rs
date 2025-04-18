use bevy::prelude::*;

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum ClientState {
    #[default]
    LoadingMain,
    Menus,
    InGame,
}

#[derive(Debug, Default, SubStates, Hash, PartialEq, Eq, Clone)]
#[source(ClientState = ClientState::InGame)]
pub enum InGamePauseState {
    #[default]
    None,
    Paused,
}

#[derive(Debug, Default, SubStates, Hash, PartialEq, Eq, Clone)]
#[source(ClientState = ClientState::InGame)]
pub enum InGameDebugState {
    #[default]
    Disabled,
    Enabled,
}

fn enter_menus(mut state: ResMut<NextState<ClientState>>) {
    state.set(ClientState::Menus);
}

pub struct ClientStatePlugin;

impl Plugin for ClientStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>()
            .add_sub_state::<InGamePauseState>()
            .add_sub_state::<InGameDebugState>()
            .add_systems(PostStartup, enter_menus);
    }
}
