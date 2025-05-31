use crate::prelude::*;

use super::io::ClientNetworkEvent;

impl NetworkClient {
    async fn handle_network_events(&mut self) -> Result<(), ClientError> {
        if let Some(event) = self.network_events.receiver.try_iter().next() {
            match event {
                ClientNetworkEvent::Connected => {
                    debug!("Network Event: Connected to server");
                    info!("Connected to server; sending hello");
                    self.send_message(ClientHello::default()).await?;
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
}

impl ClientGame {
    pub async fn update_network_client(&self) -> Result<(), ClientError> {
        let mut network_client = self.world.network_client().write().await;
        network_client.handle_connection_event();
        network_client.handle_network_events().await?;

        match network_client.connection_state {
            NetworkingState::Disconnected => {
                if let Some(command) = &self.world.client_cli_arguments().command {
                    match command {
                        ClientCliSubcommand::JoinServer(join_server) => {
                            let url = match &join_server.server_url {
                                Some(url) => url,
                                None => {
                                    &self
                                        .world
                                        .config()
                                        .read()
                                        .await
                                        .networking
                                        .default_server_address
                                }
                            };

                            network_client.connect(url).await?;
                            network_client.connection_state = NetworkingState::AttemptingConnection;
                        }
                    }
                }
            }
            // Handled in `Self::handle_network_events`
            NetworkingState::AttemptingConnection => (),
            NetworkingState::AwaitingHello => {
                if let Some(generic_message) = network_client.pop_message(self.world).await? {
                    match generic_message {
                        ServerMessageGeneric::Hello(message) => {
                            debug!("{message:#?}");
                            debug!("Reserving player slot");

                            let player_name = {
                                let config = self.world.config().read().await;
                                config.game.player_name.clone()
                            };

                            network_client
                                .send_message(ClientReserveSlot { player_name })
                                .await?;
                            network_client.connection_state = NetworkingState::ReserveSlot;
                        }
                        ServerMessageGeneric::IncompatibleProtocol(_) => {
                            error!("Server doesn't support client's protocol; disconnecting...");
                            network_client.disconnect();
                        }
                        ServerMessageGeneric::PasswordRequest(_) => {
                            // TODO: Add password prompt
                            let password = GGStringShort::try_from("1234".to_string()).unwrap();
                            debug!("Sending password to server...");
                            network_client.send(&password).await?;
                        }
                        ServerMessageGeneric::PasswordWrong(_) => {
                            error!("Server password is wrong");
                            network_client.disconnect();
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::ReserveSlot => {
                if let Some(generic_message) = network_client.pop_message(self.world).await? {
                    match generic_message {
                        ServerMessageGeneric::ServerFull(_) => {
                            info!("Server full");
                            network_client.disconnect();
                        }
                        ServerMessageGeneric::ReserveSlot(_) => {
                            debug!("Reserved player slot; joining");
                            network_client.send_message(ClientPlayerJoin).await?;
                            network_client.connection_state = NetworkingState::PlayerJoining;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::PlayerJoining => {
                if let Some(generic_message) = network_client.pop_message(self.world).await? {
                    match generic_message {
                        ServerMessageGeneric::JoinUpdate(message) => {
                            info!("Successfully joined server");
                            debug!("{message:#?}");

                            self.world
                                .players()
                                .write()
                                .await
                                .set_client_player(message.client_player_id);

                            network_client.connection_state = NetworkingState::InGame;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::InGame => {
                if let Some(generic_message) = network_client.pop_message(self.world).await? {
                    self.event_in_game(generic_message).await?;
                }
            }
        }

        Ok(())
    }
}
