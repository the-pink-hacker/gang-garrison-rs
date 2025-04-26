use gg2_client::networking::{message::server::ServerMessageGeneric, state::NetworkingState};
use gg2_common::networking::{error::Error as NetworkError, message::*};

use crate::prelude::*;

use super::io::{ClientNetworkEvent, DEFAULT_PORT};

impl NetworkClient {
    fn handle_network_events(&mut self) -> Result<()> {
        let events = self.network_events.receiver.try_iter().collect::<Vec<_>>();

        for event in events {
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

    async fn update_in_game(&mut self) -> Result<()> {
        if let Some(generic_message) = self.pop_message().await? {
            match generic_message {
                ServerMessageGeneric::CapsUpdate(message) => debug!("{:#?}", message),
                ServerMessageGeneric::ChangeMap(message) => debug!("{:#?}", message),
                ServerMessageGeneric::DropIntel(message) => debug!("{:#?}", message),
                ServerMessageGeneric::FullUpdate(message) => debug!("{:#?}", message),
                ServerMessageGeneric::InputState(message) => trace!("{:#?}", message),
                ServerMessageGeneric::MessageString(message) => {
                    info!("Server Message: \"{}\"", message.message)
                }
                ServerMessageGeneric::PlayerChangeClass(message) => debug!("{:#?}", message),
                ServerMessageGeneric::PlayerChangeTeam(message) => debug!("{:#?}", message),
                ServerMessageGeneric::PlayerJoin(message) => debug!("{:#?}", message),
                ServerMessageGeneric::QuickUpdate(message) => trace!("{:#?}", message),
                ServerMessageGeneric::WeaponFire(message) => trace!("{:#?}", message),
                _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
            }
        }

        Ok(())
    }
}

impl UpdateMutRunnable for NetworkClient {
    async fn update_mut(&mut self, world: &World) -> Result<()> {
        self.handle_connection_event();
        self.handle_network_events()?;

        match self.connection_state {
            NetworkingState::Disconnected => {
                self.connect(std::net::SocketAddr::V4(std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::LOCALHOST,
                    DEFAULT_PORT,
                )))
                .await?;
                self.connection_state = NetworkingState::AttemptingConnection;
            }
            // Handled in `Self::handle_network_events`
            NetworkingState::AttemptingConnection => (),
            NetworkingState::AwaitingHello => {
                if let Some(generic_message) = self.pop_message().await? {
                    match generic_message {
                        ServerMessageGeneric::Hello(message) => {
                            debug!("{:#?}", message);
                            debug!("Reserving player slot");
                            self.send_message(ClientReserveSlot {
                                player_name: "Rust Client".to_string().try_into().unwrap(),
                            })?;
                            self.connection_state = NetworkingState::ReserveSlot;
                        }
                        ServerMessageGeneric::PasswordRequest(_) => {
                            let password = GGStringShort::try_from("1234".to_string()).unwrap();
                            debug!("Sending password to server \"{}\"", password);
                            self.send(&password)?;
                        }
                        ServerMessageGeneric::PasswordWrong(_) => {
                            info!("Server password is wrong");
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
                            debug!("Reserving player slot");
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
                            debug!("{:#?}", message);
                            self.connection_state = NetworkingState::InGame;
                        }
                        _ => Err(NetworkError::IncorrectMessage(generic_message.into()))?,
                    }
                }
            }
            NetworkingState::InGame => self.update_in_game().await?,
        }

        Ok(())
    }
}
