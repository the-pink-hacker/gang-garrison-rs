use std::sync::Arc;

// The render thread doesn't like tokio::sync::RwLock
use async_lock::RwLock;
use tokio::time::Duration;

use crate::{camera::Camera, networking::io::NetworkClient, prelude::*};

const GAME_TPS: f32 = 60.0;
const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

pub struct App {
    world: Arc<World>,
}

impl App {
    /// Initializes the client and begins the game loop
    pub fn new() -> Self {
        env_logger::init();

        Self {
            world: Arc::default(),
        }
    }

    pub async fn start(self) -> Result<()> {
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

    async fn update(world: &World) -> Result<()> {
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
#[derive(Default)]
pub struct World {
    pub network_client: RwLock<NetworkClient>,
    pub camera: RwLock<Camera>,
}

pub trait UpdateRunnable {
    async fn update(&self, world: &World) -> Result<()>;
}

pub trait UpdateMutRunnable {
    async fn update_mut(&mut self, world: &World) -> Result<()>;
}
