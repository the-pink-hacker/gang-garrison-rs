use std::time::Duration;

use crate::prelude::*;

const GAME_TPS: f32 = 60.0;
const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

#[derive(Debug)]
pub struct ClientGame {
    pub world: &'static ClientWorld,
    pub game: CommonGame<ClientWorld>,
}

impl ClientGame {
    pub async fn start_update(self) {
        let mut interval = tokio::time::interval(Duration::from_secs_f32(GAME_LOOP_INTERVAL));

        loop {
            interval.tick().await;

            if let Err(error) = self.update().await {
                error!("{error}");
            }
        }
    }

    async fn update(&self) -> Result<(), ClientError> {
        self.update_network_client().await?;
        self.update_camera().await?;

        Ok(())
    }

    pub async fn event_in_game(&self, message: ServerMessageGeneric) -> Result<(), ClientError> {
        match message {
            ServerMessageGeneric::ChangeMap(message) => self.event_map_change(message).await?,
            _ => self.game.event_in_game(message).await?,
        }

        Ok(())
    }
}
