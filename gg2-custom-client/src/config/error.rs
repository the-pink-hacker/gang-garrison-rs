use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Failed to read config at '{1}': {0}")]
    Io(std::io::Error, PathBuf),
    #[error("Failed to deserialize config at '{1}': {0}")]
    Toml(toml::de::Error, PathBuf),
    #[error("Failed to get default config path")]
    DefaultPath,
}

#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    #[error("Failed to serialize config at '{1}': {0}")]
    Toml(toml::ser::Error, PathBuf),
    #[error("Failed to save config at '{1}': {0}")]
    Io(std::io::Error, PathBuf),
}
