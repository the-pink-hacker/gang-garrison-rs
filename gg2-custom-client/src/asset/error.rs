use std::path::PathBuf;

use gg2_custom_common::prelude::MapIoError;

use crate::prelude::*;

pub type Result<T> = std::result::Result<T, AssetError>;

#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    #[error("Failed to load asset: {0}")]
    Load(#[from] std::io::Error),
    #[error("Failed to parse texture: {0}")]
    ParseTexture(#[from] image::ImageError),
    #[error("Failed to load \"{0}\"")]
    PackMeta(PathBuf),
    #[error("Failed to parse \"{0}\": {1}")]
    PackMetaToml(PathBuf, toml::de::Error),
    #[error("Failed to strip \"{0}\" from \"{1}\"")]
    StripPrefix(PathBuf, PathBuf),
    #[error("Invalid asset path: {0}")]
    InvalidAssetPath(PathBuf),
    #[error("Failed to parse asset id's namespace")]
    IdNamespace(String),
    #[error("Toml Error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Atlas Error: {0}")]
    Atlas(#[from] image_atlas::AtlasError),
    #[error("Texture atlas is empty")]
    AtlasEmpty,
    #[error("Failed to lookup sprite {0} in atlas")]
    AtlasLookup(AssetId),
    #[error("Map Error: {0}")]
    MapIo(#[from] MapIoError),
    #[error("Asset, of type {0:?}, isn't loaded: {1}")]
    Unloaded(AssetType, AssetId),
}
