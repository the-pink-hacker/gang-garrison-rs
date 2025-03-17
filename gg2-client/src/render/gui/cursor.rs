use bevy::{
    prelude::*,
    window::WindowCreated,
    winit::cursor::{CursorIcon, CustomCursor},
};

const CURSOR_SIZE: u16 = 16;

// TODO: Seems to not work on Linux (maybe not at all)
fn setup_cursor_system(
    mut commands: Commands,
    mut window_created_events: EventReader<WindowCreated>,
    asset_server: Res<AssetServer>,
) {
    for WindowCreated { window } in window_created_events.read() {
        if let Some(mut window_commands) = commands.get_entity(*window) {
            let cursor_texture = asset_server.load("sprites/gui/crosshair.png");

            window_commands.insert(CursorIcon::Custom(CustomCursor::Image {
                handle: cursor_texture,
                hotspot: (CURSOR_SIZE / 2, CURSOR_SIZE / 2),
            }));

            debug!("Set custom game cursor");
        } else {
            error!("Failed to lookup window entity for setting game cursor");
        }
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_cursor_system);
    }
}
