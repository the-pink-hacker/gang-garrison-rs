#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum NetworkingState {
    #[default]
    Disconnected,
    AttemptingConnection,
    AwaitingHello,
    ReserveSlot,
    PlayerJoining,
    InGame,
}
