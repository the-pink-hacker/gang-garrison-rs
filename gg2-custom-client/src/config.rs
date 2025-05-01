use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use async_lock::RwLock;
use serde::{Deserialize, Serialize};
use winit::keyboard::KeyCode;

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

    /// Doesn't override unknown values
    #[serde(flatten)]
    _extra: toml::Table,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigGame {
    pub player_name: String,
}

impl Default for ClientConfigGame {
    fn default() -> Self {
        Self {
            player_name: "Rust Player".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigNetworking {
    pub default_server_address: SocketAddr,
}

impl Default for ClientConfigNetworking {
    fn default() -> Self {
        Self {
            default_server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8190),
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

pub struct ClientConfigLock(RwLock<ClientConfig>);

impl ClientConfigLock {
    pub async fn read(&self) -> async_lock::RwLockReadGuard<'_, ClientConfig> {
        self.0.read().await
    }

    /// Saves config after gaurd is dropped
    pub async fn write(&self) -> ClientConfigLockWriteGuard {
        ClientConfigLockWriteGuard(self.0.write().await)
    }
}

impl From<ClientConfig> for ClientConfigLock {
    fn from(value: ClientConfig) -> Self {
        Self(RwLock::new(value))
    }
}

pub struct ClientConfigLockWriteGuard<'a>(async_lock::RwLockWriteGuard<'a, ClientConfig>);

impl Drop for ClientConfigLockWriteGuard<'_> {
    fn drop(&mut self) {
        self.0.save().expect("Failed to save client config");
    }
}
