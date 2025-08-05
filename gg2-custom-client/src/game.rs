use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;

use crate::prelude::*;

pub const GAME_TPS: f32 = 60.0;
pub const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

pub struct ClientGame {
    pub world: &'static ClientWorld,
    pub game: CommonGame,
    pub debug_menu_button_pressed_last_frame: bool,
    pub channel: UnboundedReceiver<ClientGameMessage>,
}

impl ClientGame {
    pub fn new(world: &'static ClientWorld, channel: UnboundedReceiver<ClientGameMessage>) -> Self {
        Self {
            world,
            game: CommonGame { world },
            debug_menu_button_pressed_last_frame: false,
            channel,
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
        self.check_debug_menu_input().await;
        self.handle_client_events().await?;
        self.update_network_client().await?;

        Ok(())
    }

    pub async fn send_input_state(&self) -> Result<(), ClientError> {
        let mut key_state = KeyState::default();
        let look_axis;

        {
            let config = self.world.config().read().await;
            let controls = &config.controls;
            let input_state = self.world.input_state().read().await;

            if let Some(move_amount) = self
                .get_axis_2d(
                    &input_state,
                    &controls.move_horizontal,
                    &controls.move_vertical,
                )
                .await
            {
                match move_amount.x {
                    ..0.0 => key_state.set_left(true),
                    0.0 => (),
                    0.0.. => key_state.set_right(true),
                    _ => (),
                }

                match move_amount.y {
                    ..0.0 => key_state.set_down(true),
                    0.0 => (),
                    0.0.. => key_state.set_up(true),
                    _ => (),
                }
            } else {
                key_state.set_up(self.is_button_pressed(&input_state, &controls.up).await);
                key_state.set_down(self.is_button_pressed(&input_state, &controls.down).await);
                key_state.set_left(self.is_button_pressed(&input_state, &controls.left).await);
                key_state.set_right(self.is_button_pressed(&input_state, &controls.right).await);
            }

            key_state.set_primary(self.is_button_pressed(&input_state, &controls.fire).await);
            key_state.set_secondary(
                self.is_button_pressed(&input_state, &controls.special)
                    .await,
            );

            look_axis = self
                .get_axis_2d(
                    &input_state,
                    &controls.look_horizontal,
                    &controls.look_vertical,
                )
                .await;
        }

        let aim_direction;
        let aim_distance;

        {
            let look_position = if let Some(look) = look_axis {
                look
            } else {
                self.world
                    .winit_input_state()
                    .read()
                    .await
                    .get_mouse_position()
            };

            // TODO: Clamp aim distance to 255 * 2 (u8::MAX * fixed scale)
            aim_distance = look_position.length();
            aim_direction = if look_position == Vec2::ZERO {
                0
            } else {
                use std::f32::consts::TAU;
                let angle_radians = TAU - ((look_position.to_angle() + TAU) % TAU);
                ((angle_radians / TAU) * u16::MAX as f32).trunc() as u16
            };
        }

        self.send_client_message(ClientMessageGeneric::InputState(ClientInputState {
            input: RawInput {
                key_state,
                aim_direction,
                aim_distance,
            },
        }))
        .await?;

        Ok(())
    }

    async fn get_axis_2d(
        &self,
        input_state: &InputState,
        bind_x: &InputAxisBind,
        bind_y: &InputAxisBind,
    ) -> Option<Vec2> {
        if let Some(result_x) = input_state.poll_axis_bind(bind_x, self.world).await
            && let Some(result_y) = input_state.poll_axis_bind(bind_y, self.world).await
        {
            Some(Vec2::new(*result_x, *result_y))
        } else {
            None
        }
    }

    async fn is_button_pressed(&self, input_state: &InputState, bind: &InputButtonBind) -> bool {
        input_state
            .poll_button_bind(bind, self.world)
            .await
            .as_ref()
            .map(InputButtonResult::is_pressed)
            .unwrap_or_default()
    }

    pub async fn check_debug_menu_input(&mut self) {
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
    }

    pub async fn handle_client_events(&mut self) -> Result<(), ClientError> {
        while let Ok(message) = self.channel.try_recv() {
            match message {
                ClientGameMessage::GilrsEvent(event) => self.handle_gilrs_event(event).await?,
                ClientGameMessage::SendClientMessage(client_message) => {
                    self.send_client_message(client_message).await?
                }
            }
        }

        Ok(())
    }

    /// Client recieved server message
    pub async fn server_message(&self, message: ServerMessageGeneric) -> Result<(), ClientError> {
        match message {
            ServerMessageGeneric::ChangeMap(message) => self.event_map_change(message).await?,
            _ => self.game.server_message(message).await?,
        }

        Ok(())
    }

    /// Send message to internal and external server
    pub async fn send_client_message(
        &self,
        message: ClientMessageGeneric,
    ) -> Result<(), ClientError> {
        // TODO: Handle if the client is also the host
        self.world
            .network_client()
            .read()
            .await
            .send_message(message.clone())
            .await?;

        let player_id = self.world.client_players().read().await.get_client_id()?;
        self.game.client_message(message, player_id).await?;

        Ok(())
    }
}
