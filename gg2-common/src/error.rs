use crate::player::PlayerId;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to lookup player with index: {0}")]
    PlayerLookup(PlayerId),
    #[error("Player id is too big {0}")]
    PlayerIdOutOfBounds(std::num::TryFromIntError),
    #[error("Spectators can't spawn")]
    SpawnSpectator,
    #[error("Failed to locate spawn of index: {0}")]
    SpawnLookup(u8),
}
