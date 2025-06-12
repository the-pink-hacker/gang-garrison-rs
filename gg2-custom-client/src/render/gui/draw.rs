use super::GuiRenderer;
use crate::prelude::*;

const DEBUG_MENU_TRANSPARENCY: f32 = 0.75;
const DEBUG_MENU_TRANSPARENCY_U8: u8 = (DEBUG_MENU_TRANSPARENCY * u8::MAX as f32) as u8;

impl GuiRenderer {
    pub fn draw(&mut self, ctx: &egui::Context) {
        self.draw_debug(ctx);
    }

    fn draw_debug(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_black_alpha(DEBUG_MENU_TRANSPARENCY_U8)),
            )
            .show(ctx, |ui| {
                ui.collapsing("Client Player", |ui| self.draw_debug_player(ui));

                if ui.button("Exit Game").clicked() {
                    self.world
                        .game_to_render_channel()
                        .send(GameToRenderMessage::ExitNextFrame)
                        .expect("Failed to send exit message to render thread");
                }
            });
    }

    fn draw_debug_player(&mut self, ui: &mut egui::Ui) {
        let player = if let Ok(player) = poll_promise::Promise::spawn_async(async {
            self.world.players().read().await.get_client().cloned()
        })
        .block_and_take()
        {
            player
        } else {
            ui.label("Client player not found.");
            return;
        };

        ui.label(format!("Player Name: {}", player.name));
        ui.label(format!("Player Team: {}", player.team));
        ui.label(format!("Player Class: {}", player.class));

        //let current_team = team_selection.deref_mut();
        let current_team = &mut Team::default();

        egui::containers::ComboBox::from_label("Select Player Team")
            .selected_text(current_team.to_string())
            .show_ui(ui, |ui| {
                for team in enum_iterator::all::<Team>() {
                    ui.selectable_value(current_team, team, team.to_string());
                }
            });

        if ui.button("Update Team").clicked() {
            //if let Err(error) = network_client.send_message(ClientPlayerChangeTeam {
            //    team: *current_team,
            //}) {
            //    error!("Failed to send client player change team: {}", error);
            //}
        }

        //let current_class = class_selection.deref_mut();
        let current_class = &mut ClassGeneric::default();

        egui::containers::ComboBox::from_label("Select Player Class")
            .selected_text(current_class.to_string())
            .show_ui(ui, |ui| {
                for class in enum_iterator::all::<ClassGeneric>() {
                    ui.selectable_value(current_class, class, class.to_string());
                }
            });

        if ui.button("Update Class").clicked() {
            //if let Err(error) = network_client.send_message(ClientPlayerChangeClass {
            //    class: *current_class,
            //}) {
            //    error!("Failed to send client player change class: {}", error);
            //}
        }
    }
}
