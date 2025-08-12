use std::{path::PathBuf, sync::Arc};

use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

pub struct ClientWorld {
    render_channel: UnboundedSender<RenderMessage>,
    client_game_channel: UnboundedSender<ClientGameMessage>,
    asset_server: RwLock<AssetServer>,
    camera: RwLock<Camera>,
    client_cli_arguments: ClientCliArguments,
    config: RwLock<ClientConfig>,
    executable_directory: PathBuf,
    map_info: RwLock<MapInfo>,
    network_client: RwLock<NetworkClient>,
    players: RwLock<ClientPlayers>,
    winit_input_state: RwLock<WinitInputState>,
    gilrs_input_state: RwLock<GilrsInputState>,
    input_state: RwLock<InputState>,
    winit_input_device: Arc<dyn InputDevice>,
    gamemode_state: RwLock<Option<Box<dyn ClientGamemodeState + Send + Sync>>>,
}

impl ClientWorld {
    #[inline]
    #[must_use]
    pub fn new(
        render_channel: UnboundedSender<RenderMessage>,
        client_game_channel: UnboundedSender<ClientGameMessage>,
        client_cli_arguments: ClientCliArguments,
        config: impl Into<RwLock<ClientConfig>>,
        executable_directory: PathBuf,
    ) -> Self {
        let winit_input_device = Arc::new(WinitInputDevice::default()) as Arc<dyn InputDevice>;

        Self {
            render_channel,
            client_game_channel,
            asset_server: AssetServer::default().into(),
            camera: Camera::default().into(),
            client_cli_arguments,
            config: config.into(),
            executable_directory,
            map_info: MapInfo::default().into(),
            network_client: NetworkClient::default().into(),
            players: ClientPlayers::default().into(),
            winit_input_state: WinitInputState::default().into(),
            gilrs_input_state: GilrsInputState::default().into(),
            input_state: InputState::new(Arc::clone(&winit_input_device)).into(),
            winit_input_device,
            // TODO: Auto dectect gamemode
            gamemode_state: RwLock::new(Some(Box::new(CaptureTheFlagState::default()))),
        }
    }

    #[inline]
    #[must_use]
    pub fn network_client(&self) -> &RwLock<NetworkClient> {
        &self.network_client
    }

    #[inline]
    #[must_use]
    pub fn render_channel(&self) -> &UnboundedSender<RenderMessage> {
        &self.render_channel
    }

    #[inline]
    pub fn client_game_channel(&self) -> &UnboundedSender<ClientGameMessage> {
        &self.client_game_channel
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
    pub fn gilrs_input_state(&self) -> &RwLock<GilrsInputState> {
        &self.gilrs_input_state
    }

    #[inline]
    #[must_use]
    pub fn winit_input_device(&self) -> &Arc<dyn InputDevice + 'static> {
        &self.winit_input_device
    }

    #[inline]
    #[must_use]
    pub fn input_state(&self) -> &RwLock<InputState> {
        &self.input_state
    }

    #[inline]
    #[must_use]
    pub fn client_players(&self) -> &RwLock<ClientPlayers> {
        &self.players
    }

    #[inline]
    #[must_use]
    pub fn client_gamemode_state(
        &self,
    ) -> &RwLock<Option<Box<dyn ClientGamemodeState + Send + Sync>>> {
        &self.gamemode_state
    }
}

impl World for ClientWorld {
    #[inline]
    fn players(&self) -> &RwLock<dyn Players + Send + Sync> {
        &self.players
    }
}
