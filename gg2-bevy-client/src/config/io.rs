use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use gg2_bevy_common::config::error::{LoadError, SaveError};
use log::{error, info, warn};

use super::ClientConfig;

impl ClientConfig {
    pub fn load_wrapped() -> Self {
        match Self::load() {
            Ok(config) => config,
            Err(error) => panic!("{}", error),
        }
    }

    pub fn load() -> Result<Self, LoadError> {
        let path = Self::default_path()?;

        match read_to_string(&path) {
            Ok(config_raw) => {
                toml::from_str(&config_raw).map_err(|error| LoadError::Toml(error, path))
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                warn!("Config file not found; defaulting config.");
                Ok(Self::from_path(path))
            }
            Err(error) => Err(LoadError::Io(error, path)),
        }
    }

    pub fn save_wrapped(&self) {
        match self.save() {
            Ok(_) => info!("Config saved to: {}", self.path.display()),
            Err(error) => error!("{}", error),
        }
    }

    pub fn save(&self) -> Result<(), SaveError> {
        let config_raw = toml::to_string_pretty(&self)
            .map_err(|error| SaveError::Toml(error, self.path.clone()))?;

        File::create(&self.path)
            .map_err(|error| SaveError::Io(error, self.path.clone()))?
            .write_all(config_raw.as_bytes())
            .map_err(|error| SaveError::Io(error, self.path.clone()))
    }

    pub fn default_path_wrapped() -> PathBuf {
        match Self::default_path() {
            Ok(path) => path,
            Err(error) => panic!("{}", error),
        }
    }

    pub fn default_path() -> Result<PathBuf, LoadError> {
        std::env::current_exe()
            .map_err(|_| LoadError::DefaultPath)?
            .parent()
            .ok_or(LoadError::DefaultPath)
            .map(|path| path.join("config.toml"))
    }
}
