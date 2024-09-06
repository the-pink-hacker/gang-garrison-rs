use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum NetworkingState {
    #[default]
    Disconnected,
    AttemptingConnection,
    AwaitingHello,
    ReserveSlot,
    PlayerJoining,
}
