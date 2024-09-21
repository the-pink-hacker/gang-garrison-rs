use bevy::{input::common_conditions::input_pressed, prelude::*};

const MOVE_SPEED: f32 = 8.0;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}

fn move_camera_down(mut query: Query<&mut Transform, With<MainCamera>>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::NEG_Y * MOVE_SPEED;
    }
}

fn move_camera_up(mut query: Query<&mut Transform, With<MainCamera>>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::Y * MOVE_SPEED;
    }
}

fn move_camera_left(mut query: Query<&mut Transform, With<MainCamera>>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::NEG_X * MOVE_SPEED;
    }
}

fn move_camera_right(mut query: Query<&mut Transform, With<MainCamera>>) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        camera_transform.translation += Vec3::X * MOVE_SPEED;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            FixedUpdate,
            (
                move_camera_down.run_if(input_pressed(KeyCode::KeyS)),
                move_camera_up.run_if(input_pressed(KeyCode::KeyW)),
                move_camera_left.run_if(input_pressed(KeyCode::KeyA)),
                move_camera_right.run_if(input_pressed(KeyCode::KeyD)),
            ),
        );
    }
}
