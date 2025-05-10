use crate::player::{PlayerId, team::TeamSpawnable};

pub type Result<T> = std::result::Result<T, CommonError>;

#[derive(Debug, thiserror::Error)]
pub enum CommonError {
    #[error("Failed to lookup player with index: {0}")]
    PlayerLookup(PlayerId),
    #[error("Player id is too big {0}")]
    PlayerIdOutOfBounds(std::num::TryFromIntError),
    #[error("Player id is none.")]
    PlayerIdInvalid,
    #[error("Too many players; can't allocate another")]
    PlayerIdTooMany,
    #[error("Spectators can't spawn")]
    SpawnSpectator,
    #[error("Failed to locate {0:?} spawn at group {1} with index {2}")]
    SpawnLookup(TeamSpawnable, u8, u8),
    #[error("Failed to lookup map data")]
    MapDataLookup,
}
