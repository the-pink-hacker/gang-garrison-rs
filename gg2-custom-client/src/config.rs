use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

use crate::prelude::*;

mod error;
mod io;

#[derive(Debug)]
pub struct ClientConfig {
    values: ClientConfigRoot,
    /// The path where the config is stored
    path: PathBuf,
}

impl Deref for ClientConfig {
    type Target = ClientConfigRoot;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for ClientConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigRoot {
    pub networking: ClientConfigNetworking,
    pub game: ClientConfigGame,
    pub controls: ClientConfigControls,
    pub assets: ClientConfigAssets,

    /// Doesn't override unknown values
    #[serde(flatten)]
    _extra: toml::Table,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigGame {
    pub player_name: GGStringShort,
}

impl Default for ClientConfigGame {
    fn default() -> Self {
        Self {
            player_name: "Rust Player"
                .to_string()
                .try_into()
                .expect("Failed to create default player name"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigNetworking {
    pub default_server_address: String,
}

impl Default for ClientConfigNetworking {
    fn default() -> Self {
        Self {
            default_server_address: format!("127.0.0.1:{}", gg2_common::networking::DEFAULT_PORT),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigControls {
    pub debug_menu: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl Default for ClientConfigControls {
    fn default() -> Self {
        Self {
            debug_menu: KeyCode::F3,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigAssets {
    pub enabled_packs: Vec<String>,
}

#[derive(Debug)]
pub struct ClientConfigLock(RwLock<ClientConfig>);

impl ClientConfigLock {
    #[inline]
    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, ClientConfig> {
        self.0.read().await
    }

    /// Saves config after gaurd is dropped
    #[inline]
    pub async fn write(&self) -> ClientConfigLockWriteGuard<'_> {
        ClientConfigLockWriteGuard(self.0.write().await)
    }
}

impl From<ClientConfig> for ClientConfigLock {
    fn from(value: ClientConfig) -> Self {
        Self(RwLock::new(value))
    }
}

#[derive(Debug)]
pub struct ClientConfigLockWriteGuard<'a>(tokio::sync::RwLockWriteGuard<'a, ClientConfig>);

impl Drop for ClientConfigLockWriteGuard<'_> {
    fn drop(&mut self) {
        self.0.save().expect("Failed to save client config");
    }
}
