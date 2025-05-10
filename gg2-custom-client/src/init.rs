use std::{path::PathBuf, sync::Arc};

// The render thread doesn't like tokio::sync::RwLock
use async_lock::RwLock;
use cli::ClientCliArguments;
use tokio::time::Duration;

use crate::{
    config::{ClientConfig, ClientConfigLock},
    networking::io::NetworkClient,
    prelude::*,
};

const GAME_TPS: f32 = 60.0;
const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;
const BUILTIN_ASSET_PACKS: [&str; 1] = ["builtin"];

pub mod cli;

pub struct App {
    world: Arc<World>,
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

        Self {
            world: Arc::new(World {
                asset_server: AssetServer::default().into(),
                camera: Camera::default().into(),
                client_cli_arguments,
                config: config.into(),
                network_client: NetworkClient::default().into(),
                executable_directory,
            }),
        }
    }

    pub async fn start(self) -> Result<(), ClientError> {
        {
            let asset_folder = self.world.executable_directory.join("assets");

            let mut enabled_packs = BUILTIN_ASSET_PACKS.map(str::to_string).to_vec();

            {
                let config = self.world.config.read().await;
                enabled_packs.extend(config.assets.enabled_packs.iter().cloned());
            }

            let packs = enabled_packs
                .into_iter()
                .map(|pack_name| asset_folder.join(pack_name))
                .collect::<Vec<_>>();

            let mut asset_server = self.world.asset_server.write().await;

            asset_server.load_packs(&packs).await?;
        }

        let world = Arc::clone(&self.world);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs_f32(GAME_LOOP_INTERVAL));
            loop {
                interval.tick().await;

                if let Err(error) = Self::update(&world).await {
                    error!("{}", error);
                }
            }
        });

        self.init_render()?;

        Ok(())
    }

    pub fn get_world(&self) -> Arc<World> {
        Arc::clone(&self.world)
    }

    async fn update(world: &World) -> Result<(), ClientError> {
        {
            let mut networking_client = world.network_client.write().await;
            networking_client.update_mut(world).await?;
        }

        {
            let mut camera = world.camera.write().await;
            camera.update_mut(world).await?;
        }

        Ok(())
    }
}

/// The world is used to pass data between threads
pub struct World {
    pub asset_server: RwLock<AssetServer>,
    pub camera: RwLock<Camera>,
    pub client_cli_arguments: ClientCliArguments,
    pub config: ClientConfigLock,
    pub network_client: RwLock<NetworkClient>,
    pub executable_directory: PathBuf,
}

pub trait UpdateMutRunnable {
    async fn update_mut(&mut self, world: &World) -> Result<(), ClientError>;
}
