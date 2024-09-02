pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to lookup player with index: {0}")]
    PlayerLookup(u8),
}
