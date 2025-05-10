use std::{
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};

use super::{
    ClientConfig,
    error::{LoadError, SaveError},
};
use crate::prelude::*;

const FILE_NAME: &str = "config.toml";

impl ClientConfig {
    pub fn load(executable_directory: &Path) -> Result<Self, LoadError> {
        let path = executable_directory.join(FILE_NAME);

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

    pub fn save(&self) -> Result<(), SaveError> {
        info!("Saving config");
        let config_raw = toml::to_string_pretty(&self.values)
            .map_err(|error| SaveError::Toml(error, self.path.clone()))?;

        File::create(&self.path)
            .map_err(|error| SaveError::Io(error, self.path.clone()))?
            .write_all(config_raw.as_bytes())
            .map_err(|error| SaveError::Io(error, self.path.clone()))
    }
}
