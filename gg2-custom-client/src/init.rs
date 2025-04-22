use std::sync::Arc;

use tokio::{sync::RwLock, time::Duration};

use crate::{networking::io::NetworkClient, prelude::*};

const GAME_TPS: f32 = 30.0;
const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

pub struct App {
    pub world: Arc<World>,
}

impl App {
    /// Initializes the client and begins the game loop
    pub fn new() -> Self {
        env_logger::init();

        let mut network_client = NetworkClient::default();

        Self {
            world: Arc::new(World {
                network_client: RwLock::new(network_client),
            }),
        }
    }

    pub async fn start(self) -> Result<()> {
        let world = Arc::clone(&self.world);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs_f32(GAME_LOOP_INTERVAL));
            loop {
                interval.tick().await;

                Self::update(&world).await;
            }
        });

        self.init_render()?;

        Ok(())
    }

    async fn update(world: &World) {
        {
            let mut networking_client = world.network_client.write().await;
            networking_client.update_mut(world).await;
        }
    }
}

/// The world is used to pass data between threads
pub struct World {
    pub network_client: RwLock<NetworkClient>,
}

pub trait UpdateRunnable {
    async fn update(&self, world: &World);
}

pub trait UpdateMutRunnable {
    async fn update_mut(&mut self, world: &World);
}
