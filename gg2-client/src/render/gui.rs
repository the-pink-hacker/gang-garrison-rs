use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod cursor;
mod input;
mod menu;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            menu::GuiMenuPlugin,
            input::GuiInputPlugin,
            //cursor::CursorPlugin,
        ));
    }
}
