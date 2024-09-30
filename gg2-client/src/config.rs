use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use toml::Table;

mod io;

#[derive(Debug, Default, Serialize, Deserialize, Resource)]
#[serde(default)]
pub struct ClientConfig {
    pub networking: ClientConfigNetworking,

    #[serde(skip, default = "ClientConfig::default_path_wrapped")]
    path: PathBuf,

    /// Doesn't override unknown values
    #[serde(flatten)]
    _extra: Table,
}

impl ClientConfig {
    fn from_path(path: PathBuf) -> Self {
        Self { path, ..default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConfigNetworking {
    pub default_server_address: SocketAddr,
}

impl Default for ClientConfigNetworking {
    fn default() -> Self {
        Self {
            default_server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), 8190),
        }
    }
}

fn save_on_startup(config: Res<ClientConfig>) {
    config.save_wrapped();
}

pub struct ClientConfigPlugin;

impl Plugin for ClientConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientConfig::load_wrapped())
            .add_systems(Startup, save_on_startup);
    }
}
