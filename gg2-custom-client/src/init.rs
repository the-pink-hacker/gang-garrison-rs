use std::{path::PathBuf, sync::OnceLock};

use tokio::sync::mpsc::UnboundedReceiver;

use crate::prelude::*;

const BUILTIN_ASSET_PACKS: [&str; 2] = ["builtin", "builtin-rs"];

static WORLD: OnceLock<ClientWorld> = OnceLock::new();

pub mod cli;

pub struct App {
    pub world: &'static ClientWorld,
    pub game_to_render_channel_receiver: UnboundedReceiver<GameToRenderMessage>,
}

impl App {
    /// Initializes the client and begins the game loop
    pub fn new() -> Self {
        env_logger::init();

        let client_cli_arguments = cli::init();

        let executable_directory = std::env::current_exe()
            .ok()
            .and_then(|mut path| if path.pop() { Some(path) } else { None })
            .expect("Failed to get current exe directory");

        let config =
            ClientConfig::load(&executable_directory).expect("Failed to load client config");
        config.save().expect("Failed to save client config");

        let (game_to_render_channel_sender, game_to_render_channel_receiver) =
            tokio::sync::mpsc::unbounded_channel();

        let world = ClientWorld::new(
            game_to_render_channel_sender,
            client_cli_arguments,
            config,
            executable_directory,
        );

        let world = WORLD.get_or_init(|| world);

        Self {
            world,
            game_to_render_channel_receiver,
        }
    }

    async fn setup(&self) -> Result<(), ClientError> {
        // When ran with Cargo, located at `./assets` relative root of the project
        // When ran on own, located at `./assets` relative to exececutable
        let asset_root = std::env::var("GG2_ASSET_ROOT")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| self.world.executable_directory().join("assets"));

        let mut enabled_packs = BUILTIN_ASSET_PACKS.map(str::to_string).to_vec();

        {
            let config = self.world.config().read().await;
            enabled_packs.extend(config.assets.enabled_packs.iter().cloned());
        }

        let packs = enabled_packs
            .into_iter()
            .map(|pack_name| asset_root.join(pack_name))
            .collect::<Vec<_>>();

        let mut asset_server = self.world.asset_server().write().await;

        asset_server.load_packs(&packs).await?;
        asset_server.push_textures(self.world)?;
        asset_server.purge_textures();

        {
            let mut input = self.world.input_state().write().await;
            input.register_device(WinitInputDevice);
        }

        Ok(())
    }

    pub fn start(self) -> Result<(), ClientError> {
        // Winit runs outside of tokio but joins it to render
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime");

        runtime.block_on(self.setup())?;

        runtime.spawn(ClientGame::new(self.world, CommonGame { world: self.world }).start_update());

        self.init_render(runtime)
    }
}
