use super::MapCompression;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Only PNG maps are supported")]
    IncorrectFormat,
    #[error("Failed to read map data: {0}")]
    ReadIO(std::io::Error),
    #[error("Unexpectedly reached end of map PNG chunks")]
    EOF,
    #[error("Incorrect chunk format")]
    ChunkFormat,
    #[error("Compression type unsuported: {0:?}")]
    CompressionType(MapCompression),
}
