use std::ops::DerefMut;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use elements::*;
use gg2_common::player::{class::ClassGeneric, team::Team};

use crate::{
    config::ClientConfig,
    state::{ClientState, InGamePauseState},
};

mod elements;

fn main_system(
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

fn pause_system(
    mut contexts: EguiContexts,
    mut exit_event: EventWriter<AppExit>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut pause_state: ResMut<NextState<InGamePauseState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Resume").clicked() {
            pause_state.set(InGamePauseState::None);
        }

        if create_confirm_button(ui, "Leave Game", "Are you sure?", "Leave Game") {
            client_state.set(ClientState::Menus);
        }

        create_quit_button(ui, &mut exit_event);
    });
}

fn in_game_debug_sytem(
    mut contexts: EguiContexts,
    config: Res<ClientConfig>,
    // TODO: Get current team
    mut team: Local<Team>,
    // TODO: Get current class,
    mut class: Local<ClassGeneric>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("In Game Debugging").show(ctx, |ui| {
        ui.label(format!("Player Name: {}", config.game.player_name));

        let current_team = team.deref_mut();

        egui::containers::ComboBox::from_label("Player Team")
            .selected_text(format!("{:?}", current_team))
            .show_ui(ui, |ui| {
                for team in enum_iterator::all::<Team>() {
                    ui.selectable_value(current_team, team, format!("{:?}", team));
                }
            });

        let current_class = class.deref_mut();

        egui::containers::ComboBox::from_label("Player Class")
            .selected_text(format!("{:?}", current_class))
            .show_ui(ui, |ui| {
                for class in enum_iterator::all::<ClassGeneric>() {
                    ui.selectable_value(current_class, class, format!("{:?}", class));
                }
            });
    });
}

pub struct GuiMenuPlugin;

impl Plugin for GuiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                main_system.run_if(in_state(ClientState::Menus)),
                pause_system.run_if(in_state(InGamePauseState::Paused)),
                in_game_debug_sytem.run_if(in_state(InGamePauseState::None)),
            ),
        );
    }
}
