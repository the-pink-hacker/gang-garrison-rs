use super::{MapCompression, MapDataTag};

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
    #[error("PNG chunk has invalid CRC; Expected: {0}, Found: {1}")]
    CorruptedData(u32, u32),
    #[error("Unknown map data tag: {0}")]
    DataTag(String),
    #[error("Missing tag: {0}")]
    DataTagMissing(MapDataTag),
    #[error("End of map data")]
    DataEOF,
    #[error("Entity deserialization error: {0}")]
    Entity(serde_hjson::Error),
}
