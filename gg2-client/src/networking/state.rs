#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::States))]
pub enum NetworkingState {
    #[default]
    Disconnected,
    AttemptingConnection,
    AwaitingHello,
    ReserveSlot,
    PlayerJoining,
    InGame,
}
