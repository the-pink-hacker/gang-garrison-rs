use std::path::PathBuf;

use string_path::SPathBuf;

use crate::prelude::*;

pub type Result<T> = std::result::Result<T, ResourceError>;

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Failed to load asset: {0}")]
    Load(#[from] std::io::Error),
    #[error("Failed to load \"{0}\"")]
    PackMeta(PathBuf),
    #[error("Failed to parse \"{0}\": {1}")]
    PackMetaToml(PathBuf, toml::de::Error),
    #[error("Failed to strip \"{0}\" from \"{1}\"")]
    StripPrefix(PathBuf, PathBuf),
    #[error("Path has non-standard characters: {0}")]
    InvalidStringPath(PathBuf),
    #[error("Invalid asset path: {0}")]
    InvalidResourcePath(SPathBuf),
    #[error("Failed to parse asset id's namespace")]
    IdNamespace(String),
    #[error("Map Error: {0}")]
    MapIo(#[from] MapIoError),
}
