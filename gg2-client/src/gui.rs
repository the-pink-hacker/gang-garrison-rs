use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

mod input;
mod menu;

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(bevy_egui::egui::Visuals {
        window_rounding: 0.0.into(),
        ..default()
    })
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, menu::GuiMenuPlugin, input::GuiInputPlugin))
            .add_systems(Startup, configure_visuals_system);
    }
}
