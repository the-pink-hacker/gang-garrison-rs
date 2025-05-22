use gg2_client::networking::{message::server::ServerMessageGeneric, state::NetworkingState};
use gg2_common::{networking::message::*, string::GGStringShort};

use crate::prelude::*;

use super::io::ClientNetworkEvent;

impl NetworkClient {
    fn handle_network_events(&mut self) -> Result<(), ClientError> {
        if let Some(event) = self.network_events.receiver.try_iter().next() {
            match event {
                ClientNetworkEvent::Connected => {
                    debug!("Network Event: Connected to server");
                    info!("Connected to server; sending hello");
                    self.send_message(ClientHello::default())?;
                    self.connection_state = NetworkingState::AwaitingHello;
                }
                ClientNetworkEvent::Disconnected => {
                    debug!("Network Event: Disconnected from server");
                    self.disconnect();
                }
                ClientNetworkEvent::Error(error) => Err(error)?,
            }
        }

        Ok(())
    }

    async fn event_player_change_class(
        message: ServerPlayerChangeClass,
        world: &World,
    ) -> Result<(), ClientError> {
        let mut players = world.players.write().await;
        let player = players.get_mut(message.player_id)?;

        debug!(
            "Player {:?} class change: {} => {}",
            player.name, player.class, message.player_class
        );

        player.class = message.player_class;

        Ok(())
    }

    async fn event_player_change_name(
        message: ServerPlayerChangeName,
        world: &World,
    ) -> Result<(), ClientError> {
        let mut players = world.players.write().await;
        let player = players.get_mut(message.player_id)?;

        debug!("Player {:?} name change: {}", player.name, message.name);

        player.name = message.name;

        Ok(())
    }

    async fn event_player_change_team(
        message: ServerPlayerChangeTeam,
        world: &World,
    ) -> Result<(), ClientError> {
        let mut players = world.players.write().await;
        let player = players.get_mut(message.player_id)?;

        debug!(
            "Player {:?} team change: {} => {}",
            player.name, player.team, message.player_team
        );

        player.team = message.player_team;

        Ok(())
    }

    async fn event_player_join(
        message: ServerPlayerJoin,
        world: &World,
    ) -> Result<(), ClientError> {
        let mut players = world.players.write().await;

        let player_id = players.player_join(Player::from_name(message.player_name))?;

        debug!(
            "Player {:?} joined with id {}",
            players.get(player_id).unwrap().name,
            player_id
        );

        Ok(())
    }

    async fn event_player_leave(
        message: ServerPlayerLeave,
        world: &World,
    ) -> Result<(), ClientError> {
        let player = world.players.write().await.remove(message.player_id)?;

        debug!("Player {:?} left", player.name);

        Ok(())
    }

    async fn event_quick_update(
        message: ServerQuickUpdate,
        world: &World,
    ) -> Result<(), ClientError> {
        let characters = message
            .player_characters
            .into_iter()
            .enumerate()
            .flat_map(|(index, character)| character.map(|character| (index, character)));

        let mut players = world.players.write().await;

        for (index, (character_input, character_info)) in characters {
            let player = players.get_mut(PlayerId::try_from(index)?)?;

            player.velocity = character_info.velocity;
            player.transform.translation = Vec3::from((character_info.translation, 0.0));
            player.input_state = character_input;

            debug!("{player:#?}");
        }

        Ok(())
    }

    async fn update_in_game(&mut self, world: &World) -> Result<(), ClientError> {
        if let Some(generic_message) = self.pop_message().await? {
            match generic_message {
                ServerMessageGeneric::CaptureUpdate(message) => debug!("{message:#?}"),
                ServerMessageGeneric::ChangeMap(message) => debug!("{message:#?}"),
                ServerMessageGeneric::DropIntel(message) => debug!("{message:#?}"),
                ServerMessageGeneric::GrabIntel(message) => debug!("{message:#?}"),
                ServerMessageGeneric::FullUpdate(message) => debug!("{message:#?}"),
                ServerMessageGeneric::InputState(message) => trace!("{message:#?}"),
                ServerMessageGeneric::MessageString(message) => {
                    info!("Server Message: {:?}", message.message)
                }
                ServerMessageGeneric::PlayerChangeClass(message) => {
                    Self::event_player_change_class(message, world).await?;
                }
                ServerMessageGeneric::PlayerChangeName(message) => {
                    Self::event_player_change_name(message, world).await?;
                }
                ServerMessageGeneric::PlayerChangeTeam(message) => {
                    Self::event_player_change_team(message, world).await?;
                }
                ServerMessageGeneric::PlayerDeath(message) => debug!("{message:#?}"),
                ServerMessageGeneric::PlayerJoin(message) => {
                    Self::event_player_join(message, world).await?;
                }
                ServerMessageGeneric::PlayerLeave(message) => {
                    Self::event_player_leave(message, world).await?;
                }
                ServerMessageGeneric::PlayerSpawn(message) => debug!("{message:#?}"),
                ServerMessageGeneric::QuickUpdate(message) => {
                    Self::event_quick_update(message, world).await?;
                }
                ServerMessageGeneric::ReturnIntel(message) => debug!("{message:#?}"),
                ServerMessageGeneric::ScoreIntel(message) => debug!("{message:#?}"),
                ServerMessageGeneric::WeaponFire(message) => trace!("{message:#?}"),
                _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
            }
        }

        Ok(())
    }
}

impl UpdateMutRunnable for NetworkClient {
    async fn update_mut(&mut self, world: &World) -> Result<(), ClientError> {
        self.handle_connection_event();
        self.handle_network_events()?;

        match self.connection_state {
            NetworkingState::Disconnected => {
                if let Some(command) = &world.client_cli_arguments.command {
                    match command {
                        ClientCliSubcommand::JoinServer(join_server) => {
                            let url = match &join_server.server_url {
                                Some(url) => url,
                                None => {
                                    &world.config.read().await.networking.default_server_address
                                }
                            };

                            self.connect(url).await?;
                            self.connection_state = NetworkingState::AttemptingConnection;
                        }
                    }
                }
            }
            // Handled in `Self::handle_network_events`
            NetworkingState::AttemptingConnection => (),
            NetworkingState::AwaitingHello => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::Hello(message) => {
                            debug!("{message:#?}");
                            debug!("Reserving player slot");

                            let player_name = {
                                let config = world.config.read().await;
                                config.game.player_name.clone()
                            };

                            self.send_message(ClientReserveSlot { player_name })?;
                            self.connection_state = NetworkingState::ReserveSlot;
                        }
                        ServerMessageGeneric::IncompatibleProtocol(_) => {
                            error!("Server doesn't support client's protocol; disconnecting...");
                            self.disconnect();
                        }
                        ServerMessageGeneric::PasswordRequest(_) => {
                            // TODO: Add password prompt
                            let password = GGStringShort::try_from("1234".to_string()).unwrap();
                            debug!("Sending password to server...");
                            self.send(&password)?;
                        }
                        ServerMessageGeneric::PasswordWrong(_) => {
                            error!("Server password is wrong");
                            self.disconnect();
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::ReserveSlot => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::ServerFull(_) => {
                            info!("Server full");
                            self.disconnect();
                        }
                        ServerMessageGeneric::ReserveSlot(_) => {
                            debug!("Reserved player slot; joining");
                            self.send_message(ClientPlayerJoin)?;
                            self.connection_state = NetworkingState::PlayerJoining;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::PlayerJoining => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::JoinUpdate(message) => {
                            info!("Successfully joined server");
                            debug!("{message:#?}");

                            world
                                .players
                                .write()
                                .await
                                .set_client_player(message.client_player_id);

                            self.connection_state = NetworkingState::InGame;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::InGame => self.update_in_game(world).await?,
        }

        Ok(())
    }
}
