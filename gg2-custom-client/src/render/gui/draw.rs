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
                        .game_to_render_channel()
                        .send(GameToRenderMessage::ExitNextFrame)
                        .expect("Failed to send exit message to render thread");
                }
            });
    }

    fn draw_debug_player(&mut self, ui: &mut egui::Ui) {
        let mut players = Promise::spawn_async(self.world.players().write()).block_and_take();
        let player = if let Ok(player) = players.get_client_mut() {
            player
        } else {
            ui.label("Client player not found");
            return;
        };

        ui.label(format!("Player Name: {}", player.name));
        ui.label(format!("Player Team: {}", player.team));
        ui.label(format!("Player Class: {}", player.class));

        egui::containers::ComboBox::from_label("Select Player Team")
            .selected_text(player.team.to_string())
            .show_ui(ui, |ui| {
                for team in enum_iterator::all::<Team>() {
                    ui.selectable_value(&mut player.team, team, team.to_string());
                }
            });

        if ui.button("Update Team").clicked() {
            let team = player.team;
            let world = self.world;
            tokio::spawn(async move {
                if let Err(error) = world
                    .network_client()
                    .read()
                    .await
                    .send_message(ClientPlayerChangeTeam { team })
                    .await
                {
                    error!("Failed to send client player change team: {error}");
                }
            });
        }

        egui::containers::ComboBox::from_label("Select Player Class")
            .selected_text(player.class.to_string())
            .show_ui(ui, |ui| {
                for class in enum_iterator::all::<ClassGeneric>() {
                    ui.selectable_value(&mut player.class, class, class.to_string());
                }
            });

        if ui.button("Update Class").clicked() {
            let class = player.class;
            let world = self.world;
            tokio::spawn(async move {
                if let Err(error) = world
                    .network_client()
                    .read()
                    .await
                    .send_message(ClientPlayerChangeClass { class })
                    .await
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
                //let player_name = config.game.player_name.clone();
                //let world = self.world;
                //poll_promise::Promise::spawn_async(async move {
                //    world.network_client().read().await.send_message()
                //});
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
