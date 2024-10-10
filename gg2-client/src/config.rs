use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod io;

#[derive(Debug, Default, Serialize, Deserialize, Resource)]
#[serde(default)]
pub struct ClientConfig {
    pub networking: ClientConfigNetworking,
    pub game: ClientConfigGame,
    pub controls: ClientConfigControls,

    #[serde(skip, default = "ClientConfig::default_path_wrapped")]
    path: PathBuf,

    /// Doesn't override unknown values
    #[serde(flatten)]
    _extra: toml::Table,
}

impl ClientConfig {
    fn from_path(path: PathBuf) -> Self {
        Self { path, ..default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigGame {
    pub player_name: String,
}

impl Default for ClientConfigGame {
    fn default() -> Self {
        Self {
            player_name: "RustBevy".to_string(),
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
            default_server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8190),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigControls {
    pub debug_menu: KeyCode,
}

impl Default for ClientConfigControls {
    fn default() -> Self {
        Self {
            debug_menu: KeyCode::F3,
        }
    }
}

fn save_config_system(config: Res<ClientConfig>) {
    config.save_wrapped();
}

pub struct ClientConfigPlugin;

impl Plugin for ClientConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClientConfig::load_wrapped())
            .add_systems(
                FixedPostUpdate,
                save_config_system.run_if(resource_changed::<ClientConfig>),
            );
    }
}
