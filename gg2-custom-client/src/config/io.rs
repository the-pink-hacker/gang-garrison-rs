use std::{
    fs::{File, read_to_string},
    io::Write,
    path::PathBuf,
};

use super::{
    ClientConfig,
    error::{LoadError, SaveError},
};
use crate::prelude::*;

impl ClientConfig {
    pub fn load() -> std::result::Result<Self, LoadError> {
        let path = Self::default_path()?;

        match read_to_string(&path) {
            Ok(config_raw) => {
                info!("Loading config");
                let values = toml::from_str(&config_raw)
                    .map_err(|error| LoadError::Toml(error, path.clone()))?;

                Ok(Self { values, path })
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                warn!("Config file not found; defaulting config");
                Ok(Self {
                    values: Default::default(),
                    path,
                })
            }
            Err(error) => Err(LoadError::Io(error, path)),
        }
    }

    pub fn save(&self) -> std::result::Result<(), SaveError> {
        info!("Saving config");
        let config_raw = toml::to_string_pretty(&self.values)
            .map_err(|error| SaveError::Toml(error, self.path.clone()))?;

        File::create(&self.path)
            .map_err(|error| SaveError::Io(error, self.path.clone()))?
            .write_all(config_raw.as_bytes())
            .map_err(|error| SaveError::Io(error, self.path.clone()))
    }

    fn default_path() -> std::result::Result<PathBuf, LoadError> {
        std::env::current_exe()
            .map_err(|_| LoadError::DefaultPath)?
            .parent()
            .ok_or(LoadError::DefaultPath)
            .map(|path| path.join("config.toml"))
    }
}
