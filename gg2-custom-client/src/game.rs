use std::time::Duration;

use crate::prelude::*;

const GAME_TPS: f32 = 60.0;
const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

#[derive(Debug)]
pub struct ClientGame {
    pub world: &'static ClientWorld,
    pub game: CommonGame<ClientWorld>,
    pub debug_menu_button_pressed_last_frame: bool,
}

impl ClientGame {
    pub fn new(world: &'static ClientWorld, game: CommonGame<ClientWorld>) -> Self {
        Self {
            world,
            game,
            debug_menu_button_pressed_last_frame: false,
        }
    }

    pub async fn start_update(mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs_f32(GAME_LOOP_INTERVAL));

        loop {
            interval.tick().await;

            if let Err(error) = self.update().await {
                error!("{error}");
            }
        }
    }

    async fn update(&mut self) -> Result<(), ClientError> {
        let debug_button_pressed = {
            let debug_bind = &self.world.config().read().await.controls.debug_menu;

            self.world
                .input_state()
                .read()
                .await
                .poll_button_bind(debug_bind, self.world)
                .await
                .unwrap_or_default()
                .is_pressed()
        };

        if debug_button_pressed {
            if !self.debug_menu_button_pressed_last_frame {
                let mut config = self.world.config().write().await;
                config.debug.gui = !config.debug.gui;

                if let Err(error) = config.save() {
                    error!("{error}");
                }
            }

            self.debug_menu_button_pressed_last_frame = true;
        } else {
            self.debug_menu_button_pressed_last_frame = false;
        }

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
