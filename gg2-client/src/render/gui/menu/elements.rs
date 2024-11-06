use bevy::prelude::*;
use bevy_egui::egui::{RichText, Ui, WidgetText};

pub fn create_quit_button(ui: &mut Ui, event_writer: &mut EventWriter<AppExit>) {
    if create_confirm_button(ui, "Quit", "Are you sure?", "Quit") {
        info!("User quiting game.");
        event_writer.send(AppExit::Success);
    }
}

/// Returns a true response when confirmed.
pub fn create_confirm_button(
    ui: &mut Ui,
    title: impl Into<WidgetText>,
    menu_title: impl Into<RichText>,
    confirm: impl Into<WidgetText>,
) -> bool {
    let mut response = false;

    ui.menu_button(title, |ui| {
        ui.heading(menu_title);

        if ui.button("Cancel").clicked() {
            ui.close_menu();
            response = false;
        }

        if ui.button(confirm).clicked() {
            response = true;
        }
    });

    response
}
