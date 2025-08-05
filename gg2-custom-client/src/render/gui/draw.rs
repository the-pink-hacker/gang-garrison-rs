use poll_promise::Promise;

use super::GuiRenderer;
use crate::prelude::*;

const DEBUG_MENU_TRANSPARENCY: f32 = 0.75;
const DEBUG_MENU_TRANSPARENCY_U8: u8 = (DEBUG_MENU_TRANSPARENCY * u8::MAX as f32) as u8;

impl GuiRenderer {
    pub fn draw(&mut self, ctx: &egui::Context) {
        let debug_ui = Promise::spawn_async(self.world.config().read())
            .block_and_take()
            .debug
            .gui;

        if debug_ui {
            self.draw_debug(ctx);
        }
    }

    fn draw_debug(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_black_alpha(DEBUG_MENU_TRANSPARENCY_U8)),
            )
            .show(ctx, |ui| {
                ui.collapsing("Client Player", |ui| self.draw_debug_player(ui));
                ui.collapsing("Config", |ui| self.draw_debug_config(ui));

                if ui.button("Exit Game").clicked() {
                    self.world
                        .render_channel()
                        .send(RenderMessage::ExitNextFrame)
                        .expect("Failed to send exit message to render thread");
                }
            });
    }

    fn draw_debug_player(&mut self, ui: &mut egui::Ui) {
        {
            let players = Promise::spawn_async(self.world.client_players().read()).block_and_take();
            let player = if let Ok(player) = players.get_client() {
                player
            } else {
                ui.label("Client player not found");
                return;
            };

            ui.label(format!("Player Name: {}", player.name));
            ui.label(format!("Player Team: {}", player.team));
            ui.label(format!("Player Class: {}", player.class));
        }

        egui::containers::ComboBox::from_label("Select Player Team")
            .selected_text(self.debug_player_team.to_string())
            .show_ui(ui, |ui| {
                for team in enum_iterator::all::<Team>() {
                    ui.selectable_value(&mut self.debug_player_team, team, team.to_string());
                }
            });

        if ui.button("Update Team").clicked() {
            let team = self.debug_player_team;
            let world = self.world;
            tokio::spawn(async move {
                if let Err(error) =
                    world
                        .client_game_channel()
                        .send(ClientGameMessage::SendClientMessage(
                            ClientMessageGeneric::PlayerChangeTeam(ClientPlayerChangeTeam { team }),
                        ))
                {
                    error!("Failed to send client player change team: {error}");
                }
            });
        }

        egui::containers::ComboBox::from_label("Select Player Class")
            .selected_text(self.debug_player_class.to_string())
            .show_ui(ui, |ui| {
                for class in enum_iterator::all::<ClassGeneric>() {
                    ui.selectable_value(&mut self.debug_player_class, class, class.to_string());
                }
            });

        if ui.button("Update Class").clicked() {
            let class = self.debug_player_class;
            let world = self.world;
            tokio::spawn(async move {
                if let Err(error) =
                    world
                        .client_game_channel()
                        .send(ClientGameMessage::SendClientMessage(
                            ClientMessageGeneric::PlayerChangeClass(ClientPlayerChangeClass {
                                class,
                            }),
                        ))
                {
                    error!("Failed to send client player change team: {error}");
                }
            });
        }
    }

    fn draw_debug_config(&mut self, ui: &mut egui::Ui) {
        let mut config =
            poll_promise::Promise::spawn_async(self.world.config().write()).block_and_take();

        ui.columns_const(|[label, text, button]| {
            label.label("Player Name");
            let text_response = text.text_edit_singleline(&mut *config.game.player_name);
            let button_response = button.small_button("Sync");

            let enter = button_response.clicked()
                || (text_response.lost_focus() && text.input(|i| i.key_pressed(egui::Key::Enter)));

            if enter {
                //if let Err(error) = self.world.client_game_channel().send(ClientGameMessage::SendClientMessage(ClientMessageGeneric::PlayerChangeName())) {
                //    error!("Failed to update player name: {error}");
                //}
                //
                todo!("Send new player name to server.");
            }
        });

        ui.columns_const(|[label, text]| {
            label.label("Default Server Address");
            text.text_edit_singleline(&mut config.networking.default_server_address);
        });

        if ui.button("Save").clicked()
            && let Err(error) = config.save()
        {
            error!("Failed to save client config: {error}");
        }
    }
}
