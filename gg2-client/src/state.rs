use bevy::prelude::*;

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
pub enum ClientState {
    #[default]
    LoadingMain,
    Menus,
    InGame,
}

fn enter_menus(mut state: ResMut<NextState<ClientState>>) {
    state.set(ClientState::Menus);
}

pub struct ClientStatePlugin;

impl Plugin for ClientStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>()
            .add_systems(PostStartup, enter_menus);
    }
}
