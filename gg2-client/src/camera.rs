use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
