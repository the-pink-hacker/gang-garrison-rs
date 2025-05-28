use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

pub struct ClientWorld {
    game_to_render_channel: UnboundedSender<GameToRenderMessage>,
    asset_server: RwLock<AssetServer>,
    camera: RwLock<Camera>,
    client_cli_arguments: ClientCliArguments,
    config: ClientConfigLock,
    executable_directory: PathBuf,
    map_info: RwLock<MapInfo>,
    network_client: RwLock<NetworkClient>,
    players: RwLock<ClientPlayers>,
}

impl ClientWorld {
    #[inline]
    pub fn new(
        game_to_render_channel: UnboundedSender<GameToRenderMessage>,
        client_cli_arguments: ClientCliArguments,
        config: impl Into<ClientConfigLock>,
        executable_directory: PathBuf,
    ) -> Self {
        Self {
            game_to_render_channel,
            asset_server: AssetServer::default().into(),
            camera: Camera::default().into(),
            client_cli_arguments,
            config: config.into(),
            executable_directory,
            map_info: MapInfo::default().into(),
            network_client: NetworkClient::default().into(),
            players: ClientPlayers::default().into(),
        }
    }

    #[inline]
    pub fn network_client(&self) -> &RwLock<NetworkClient> {
        &self.network_client
    }

    #[inline]
    pub fn game_to_render_channel(&self) -> &UnboundedSender<GameToRenderMessage> {
        &self.game_to_render_channel
    }

    #[inline]
    pub fn asset_server(&self) -> &RwLock<AssetServer> {
        &self.asset_server
    }

    #[inline]
    pub fn executable_directory(&self) -> &PathBuf {
        &self.executable_directory
    }

    #[inline]
    pub fn camera(&self) -> &RwLock<Camera> {
        &self.camera
    }

    #[inline]
    pub fn client_cli_arguments(&self) -> &ClientCliArguments {
        &self.client_cli_arguments
    }

    #[inline]
    pub fn map_info(&self) -> &RwLock<MapInfo> {
        &self.map_info
    }

    #[inline]
    pub fn config(&self) -> &ClientConfigLock {
        &self.config
    }
}

impl World for ClientWorld {
    type Players = ClientPlayers;

    #[inline]
    fn players(&self) -> &RwLock<Self::Players> {
        &self.players
    }
}
