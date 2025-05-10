use std::path::PathBuf;

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
}
