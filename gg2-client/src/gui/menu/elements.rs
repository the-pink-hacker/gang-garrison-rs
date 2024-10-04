use bevy::prelude::*;
use bevy_egui::egui::Ui;

pub fn create_quit_button(ui: &mut Ui, event_writer: &mut EventWriter<AppExit>) {
    ui.menu_button("Quit", |ui| {
        ui.heading("Are you sure?");

        if ui.button("Cancel").clicked() {
            ui.close_menu();
        }

        if ui.button("Quit").clicked() {
            println!("User quiting game.");
            event_writer.send(AppExit::Success);
        }
    });
}
