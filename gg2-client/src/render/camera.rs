use bevy::{input::common_conditions::input_pressed, prelude::*};

const MOVE_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct MainCamera;

fn setup_system(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}

fn move_camera_down_system(mut query: Query<&mut Transform, With<MainCamera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::NEG_Y * MOVE_SPEED * time.delta_seconds();
    }
}

fn move_camera_up_system(mut query: Query<&mut Transform, With<MainCamera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::Y * MOVE_SPEED * time.delta_seconds();
    }
}

fn move_camera_left_system(mut query: Query<&mut Transform, With<MainCamera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::NEG_X * MOVE_SPEED * time.delta_seconds();
    }
}

fn move_camera_right_system(mut query: Query<&mut Transform, With<MainCamera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::X * MOVE_SPEED * time.delta_seconds();
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system).add_systems(
            Update,
            (
                move_camera_down_system.run_if(input_pressed(KeyCode::KeyS)),
                move_camera_up_system.run_if(input_pressed(KeyCode::KeyW)),
                move_camera_left_system.run_if(input_pressed(KeyCode::KeyA)),
                move_camera_right_system.run_if(input_pressed(KeyCode::KeyD)),
            ),
        );
    }
}
