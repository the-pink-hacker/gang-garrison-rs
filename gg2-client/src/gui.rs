use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::{EguiContexts, EguiPlugin};

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(bevy_egui::egui::Visuals {
        window_rounding: 0.0.into(),
        ..default()
    })
}

fn ui_main_menu_system(mut contexts: EguiContexts, mut exit_event: EventWriter<AppExit>) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Gang Garrison 2: Rust");
        let _ = ui.button("Host Game");
        let _ = ui.button("Join");
        let _ = ui.button("Options");
        let _ = ui.button("Credits");

        if ui.button("Visit The Forums").clicked() {
            ctx.open_url(egui::OpenUrl::new_tab(
                "https://www.ganggarrison.com/forums/index.php",
            ));
        }

        if ui.button("Quit").clicked() {
            println!("User quiting game.");
            exit_event.send(AppExit::Success);
        }
    });
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Startup, configure_visuals_system)
            .add_systems(Update, ui_main_menu_system);
    }
}
