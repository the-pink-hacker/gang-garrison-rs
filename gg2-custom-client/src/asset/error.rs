use crate::prelude::*;

pub type Result<T> = std::result::Result<T, AssetError>;

#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("Asset IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse texture: {0}")]
    ParseTexture(#[from] image::ImageError),
    #[error("Toml Error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Atlas Error: {0}")]
    Atlas(#[from] image_atlas::AtlasError),
    #[error("Texture atlas is empty")]
    AtlasEmpty,
    #[error("Failed to lookup sprite {0} in atlas")]
    AtlasLookup(ResourceId),
    #[error("Asset, of type {0:?}, isn't loaded: {1}")]
    Unloaded(String, ResourceId),
    #[error("{0}")]
    Resource(#[from] ResourceError),
    #[error("{0}")]
    Map(#[from] MapIoError),
}
