use bevy::{
    prelude::*,
    render::view::RenderLayers,
    window::{PrimaryWindow, WindowCreated},
    winit::cursor::{CursorIcon, CustomCursor},
};

use crate::render::camera::MainCamera;

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

#[derive(Component, Default)]
pub struct Crosshair;

#[derive(Component, Default)]
#[require(Crosshair)]
pub struct MainCrosshair;

fn setup_cursor_workaround_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let cursor_texture = asset_server.load("sprites/gui/crosshair.png");

    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor_options.visible = false;
    } else {
        error!("Failed to lookup primary window.");
    }

    commands.spawn((
        MainCrosshair,
        Sprite {
            image: cursor_texture,
            ..default()
        },
        RenderLayers::layer(0),
    ));
}

fn update_cursor_position_system(
    mut crosshair_query: Query<&mut Transform, With<MainCrosshair>>,
    camera_query: Query<&GlobalTransform, With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut crosshair_position) = crosshair_query.get_single_mut() {
        if let Ok(window) = window_query.get_single() {
            if let Ok(camera_transform) = camera_query.get_single() {
                if let Some(cursor_position) = window.cursor_position() {
                    let camera_position = camera_transform.translation();
                    crosshair_position.translation.x =
                        camera_position.x - window.width() / 2.0 + cursor_position.x;
                    crosshair_position.translation.y =
                        camera_position.y + window.height() / 2.0 - cursor_position.y;
                }
            } else {
                error!("Failed to lookup main camera.");
            }
        } else {
            error!("Failed to lookup window.");
        }
    } else {
        error!("Failed to lookup main cursor.");
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor_workaround_system)
            //.add_systems(PreUpdate, setup_cursor_system)
            .add_systems(PostUpdate, update_cursor_position_system);
    }
}
