use bevy::prelude::*;

pub mod camera;
pub mod gui;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, gui::GuiPlugin));
    }
}
