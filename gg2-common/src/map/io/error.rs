use super::MapDataTag;

pub type Result<T> = std::result::Result<T, MapIoError>;

#[derive(Debug, thiserror::Error)]
pub enum MapIoError {
    #[error("Failed to decode map PNG: {0}")]
    PngDecode(#[from] png::DecodingError),
    #[error("Failed to find map data chunk")]
    Chunk,
    #[error("Incorrect map data tag: {0}")]
    DataTag(String),
    #[error("Data tag unexpected: {0}")]
    DataTagUnexpected(MapDataTag),
    #[error("Unclosed data tag; expected {expected}, but got {got}")]
    DataTagUnclosed {
        expected: MapDataTag,
        got: MapDataTag,
    },
    #[error("Reached end of map data")]
    DataEOF,
    #[error("Failed to parse map size: {0}")]
    MapSize(#[from] std::num::ParseIntError),
    #[error("Failed to parse map entity data")]
    ParseEntity(#[from] serde_hjson::Error),
    #[error("Missing map data tag: {0}")]
    DataTagMissing(MapDataTag),
}
