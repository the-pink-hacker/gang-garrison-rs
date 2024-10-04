use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use elements::*;

use crate::state::{ClientState, InGameVisualState};

mod elements;

pub fn main_system(
    mut contexts: EguiContexts,
    mut exit_event: EventWriter<AppExit>,
    mut client_state: ResMut<NextState<ClientState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Gang Garrison 2: Rust");

        ui.add_enabled(false, egui::Button::new("Host Game"));

        if ui.button("Join").clicked() {
            println!("Joining server...");
            client_state.set(ClientState::InGame);
        }

        ui.add_enabled_ui(false, |ui| {
            let _ = ui.button("Options");
            let _ = ui.button("Credits");
        });

        if ui.button("Visit The Forums").clicked() {
            ctx.open_url(egui::OpenUrl::new_tab(
                "https://www.ganggarrison.com/forums/index.php",
            ));
        }

        create_quit_button(ui, &mut exit_event);
    });
}

pub fn pause_system(
    mut contexts: EguiContexts,
    mut exit_event: EventWriter<AppExit>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut pause_state: ResMut<NextState<InGameVisualState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Resume").clicked() {
            pause_state.set(InGameVisualState::None);
        }

        if create_confirm_button(ui, "Leave Game", "Are you sure?", "Leave Game") {
            client_state.set(ClientState::Menus);
        }

        create_quit_button(ui, &mut exit_event);
    });
}

pub struct GuiMenuPlugin;

impl Plugin for GuiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                main_system.run_if(in_state(ClientState::Menus)),
                pause_system.run_if(in_state(InGameVisualState::Paused)),
            ),
        );
    }
}
