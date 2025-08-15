use std::time::Duration;

use tokio::time::Instant;

use crate::prelude::*;

pub const GAME_TPS: f32 = 60.0;
pub const GAME_LOOP_INTERVAL: f32 = 1.0 / GAME_TPS;

pub mod gamemode;
pub mod world;

pub struct CommonGame {
    pub world: &'static dyn World,
    last_tick: Instant,
}

impl CommonGame {
    pub fn new(world: &'static dyn World) -> Self {
        let last_tick = Instant::now() - Duration::from_secs_f32(GAME_LOOP_INTERVAL);

        Self { world, last_tick }
    }

    pub async fn pre_tick(&mut self) -> Result<(), CommonError> {
        self.update_tick_delta().await;

        Ok(())
    }

    pub async fn tick(&mut self) -> Result<(), CommonError> {
        self.world
            .with_gamemode_state_mut(Box::new(|gamemode_state| {
                Box::pin(async {
                    if let Some(gamemode_state) = gamemode_state {
                        gamemode_state.tick(self.world).await
                    } else {
                        Ok(())
                    }
                })
            }))
            .await?;

        Ok(())
    }

    async fn update_tick_delta(&mut self) {
        let delta = self.last_tick.elapsed();

        self.world.set_delta_tick(delta.as_secs_f32());

        self.last_tick = Instant::now();
    }

    /// Handles message from the client
    pub async fn client_message(
        &self,
        generic_message: ClientMessageGeneric,
        player_id: PlayerId,
    ) -> Result<(), CommonError> {
        match generic_message {
            ClientMessageGeneric::Hello(message) => debug!("{message:#?}"),
            ClientMessageGeneric::InputState(message) => {
                debug!("Player {player_id}: {message:#?}");
            }
            ClientMessageGeneric::PlayerChangeClass(message) => {
                debug!("Player {player_id}: {message:#?}");
                self.world.players().write().await.get_mut(player_id)?.class = message.class;
            }
            ClientMessageGeneric::PlayerChangeTeam(message) => {
                debug!("Player {player_id}: {message:#?}");
                self.world.players().write().await.get_mut(player_id)?.team = message.team;
            }
            ClientMessageGeneric::PlayerJoin(message) => debug!("{message:#?}"),
            ClientMessageGeneric::ReserveSlot(message) => debug!("{message:#?}"),
        }

        Ok(())
    }

    /// Handles message from the server
    pub async fn server_message(
        &self,
        generic_message: ServerMessageGeneric,
    ) -> Result<(), CommonError> {
        match generic_message {
            ServerMessageGeneric::CaptureUpdate(message) => debug!("{message:#?}"),
            ServerMessageGeneric::ChatBubble(message) => debug!("{message:#?}"),
            ServerMessageGeneric::DropIntel(message) => debug!("{message:#?}"),
            ServerMessageGeneric::GrabIntel(message) => debug!("{message:#?}"),
            ServerMessageGeneric::FullUpdate(message) => debug!("{message:#?}"),
            ServerMessageGeneric::InputState(message) => {
                self.event_input_state(message).await?;
            }
            ServerMessageGeneric::MessageString(message) => {
                info!("Server Message: {:?}", message.message);
            }
            ServerMessageGeneric::Omnom(message) => debug!("{message:#?}"),
            ServerMessageGeneric::PlayerChangeClass(message) => {
                self.event_player_change_class(message).await?;
            }
            ServerMessageGeneric::PlayerChangeName(message) => {
                self.event_player_change_name(message).await?;
            }
            ServerMessageGeneric::PlayerChangeTeam(message) => {
                self.event_player_change_team(message).await?;
            }
            ServerMessageGeneric::PlayerDeath(message) => debug!("{message:#?}"),
            ServerMessageGeneric::PlayerJoin(message) => {
                self.event_player_join(message).await?;
            }
            ServerMessageGeneric::PlayerLeave(message) => {
                self.event_player_leave(message).await?;
            }
            ServerMessageGeneric::PlayerSpawn(message) => debug!("{message:#?}"),
            ServerMessageGeneric::QuickUpdate(message) => {
                self.event_quick_update(message).await?;
            }
            ServerMessageGeneric::ReturnIntel(message) => debug!("{message:#?}"),
            ServerMessageGeneric::ScoreIntel(message) => debug!("{message:#?}"),
            ServerMessageGeneric::WeaponFire(message) => {
                self.event_weapon_fire(message).await?;
            }
            _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
        }

        Ok(())
    }

    async fn event_weapon_fire(&self, message: ServerWeaponFire) -> Result<(), CommonError> {
        debug!("{message:#?}");

        let mut players = self.world.players().write().await;
        let player = players.get_mut(message.player_id)?;

        // TODO: Verify if player has character
        player.transform.translation.x = message.position.x;
        player.transform.translation.y = message.position.y;
        player.velocity = message.velocity;

        // TODO: Implement weapons

        Ok(())
    }

    async fn event_input_state(&self, message: ServerInputState) -> Result<(), CommonError> {
        trace!("{message:#?}");

        self.world
            .players()
            .write()
            .await
            .flat_zip_mut(message.inputs)
            .for_each(|(player, input)| Self::apply_player_raw_input(player, input));

        Ok(())
    }

    async fn event_player_change_class(
        &self,
        message: ServerPlayerChangeClass,
    ) -> Result<(), CommonError> {
        let mut players = self.world.players().write().await;
        let player = players.get_mut(message.player_id)?;

        debug!(
            "Player {:?} class change: {} => {}",
            player.name, player.class, message.player_class
        );

        player.class = message.player_class;

        Ok(())
    }

    async fn event_player_change_name(
        &self,
        message: ServerPlayerChangeName,
    ) -> Result<(), CommonError> {
        let mut players = self.world.players().write().await;
        let player = players.get_mut(message.player_id)?;

        debug!("Player {:?} name change: {}", player.name, message.name);

        player.name = message.name;

        Ok(())
    }

    async fn event_player_change_team(
        &self,
        message: ServerPlayerChangeTeam,
    ) -> Result<(), CommonError> {
        let mut players = self.world.players().write().await;
        let player = players.get_mut(message.player_id)?;

        debug!(
            "Player {:?} team change: {} => {}",
            player.name, player.team, message.player_team
        );

        player.team = message.player_team;

        Ok(())
    }

    async fn event_player_join(&self, message: ServerPlayerJoin) -> Result<(), CommonError> {
        let mut players = self.world.players().write().await;

        let player_id = players.push(Player::from_name(message.player_name))?;

        debug!(
            "Player {:?} joined with id {}",
            players.get(player_id).unwrap().name,
            player_id
        );

        Ok(())
    }

    async fn event_player_leave(&self, message: ServerPlayerLeave) -> Result<(), CommonError> {
        let player = self
            .world
            .players()
            .write()
            .await
            .remove(message.player_id)?;

        debug!("Player {:?} left", player.name);

        Ok(())
    }

    async fn event_quick_update(&self, message: ServerQuickUpdate) -> Result<(), CommonError> {
        let mut players = self.world.players().write().await;

        for (player, (character_input, character_info)) in
            players.flat_zip_mut(message.player_characters)
        {
            player.velocity = character_info.velocity;
            player.transform.translation = Vec3::from((character_info.translation, 0.0));

            Self::apply_player_raw_input(player, character_input);
        }

        Ok(())
    }

    fn apply_player_raw_input(player: &mut Player, raw_input: RawInput) {
        player.input_state = raw_input;

        let scale_x = if player.input_state.looking_left() {
            -crate::player::PLAYER_SCALE
        } else {
            crate::player::PLAYER_SCALE
        };

        player.transform.scale.x = scale_x;
    }
}
