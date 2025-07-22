use std::path::PathBuf;

use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

#[derive(Debug)]
pub struct ClientWorld {
    game_to_render_channel: UnboundedSender<GameToRenderMessage>,
    asset_server: RwLock<AssetServer>,
    camera: RwLock<Camera>,
    client_cli_arguments: ClientCliArguments,
    config: RwLock<ClientConfig>,
    executable_directory: PathBuf,
    map_info: RwLock<MapInfo>,
    network_client: RwLock<NetworkClient>,
    players: RwLock<ClientPlayers>,
    winit_input_state: RwLock<WinitInputState>,
    input_state: RwLock<InputState>,
}

impl ClientWorld {
    #[inline]
    #[must_use]
    pub fn new(
        game_to_render_channel: UnboundedSender<GameToRenderMessage>,
        client_cli_arguments: ClientCliArguments,
        config: impl Into<RwLock<ClientConfig>>,
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
            winit_input_state: WinitInputState::default().into(),
            input_state: InputState::default().into(),
        }
    }

    #[inline]
    #[must_use]
    pub fn network_client(&self) -> &RwLock<NetworkClient> {
        &self.network_client
    }

    #[inline]
    #[must_use]
    pub fn game_to_render_channel(&self) -> &UnboundedSender<GameToRenderMessage> {
        &self.game_to_render_channel
    }

    #[inline]
    #[must_use]
    pub fn asset_server(&self) -> &RwLock<AssetServer> {
        &self.asset_server
    }

    #[inline]
    #[must_use]
    pub fn executable_directory(&self) -> &PathBuf {
        &self.executable_directory
    }

    #[inline]
    #[must_use]
    pub fn camera(&self) -> &RwLock<Camera> {
        &self.camera
    }

    #[inline]
    #[must_use]
    pub fn client_cli_arguments(&self) -> &ClientCliArguments {
        &self.client_cli_arguments
    }

    #[inline]
    #[must_use]
    pub fn map_info(&self) -> &RwLock<MapInfo> {
        &self.map_info
    }

    #[inline]
    #[must_use]
    pub fn config(&self) -> &RwLock<ClientConfig> {
        &self.config
    }

    #[inline]
    #[must_use]
    pub fn winit_input_state(&self) -> &RwLock<WinitInputState> {
        &self.winit_input_state
    }

    #[inline]
    #[must_use]
    pub fn input_state(&self) -> &RwLock<InputState> {
        &self.input_state
    }
}

impl World for ClientWorld {
    type Players = ClientPlayers;

    #[inline]
    fn players(&self) -> &RwLock<Self::Players> {
        &self.players
    }
}
