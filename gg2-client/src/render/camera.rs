use bevy::{input::common_conditions::input_pressed, prelude::*, window::WindowResized};

const MOVE_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct MainCamera;

fn setup_system(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera: Camera {
                viewport: Some(default()),
                ..default()
            },
            ..default()
        },
    ));
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

fn crop_aspect_ratio(
    ratio_width: u32,
    ratio_height: u32,
    window_width: u32,
    window_height: u32,
) -> UVec2 {
    let (width, height) = if window_width > window_height {
        let ratio = ratio_width as f32 / ratio_height as f32;
        let width = (window_height as f32 * ratio) as u32;
        (width, window_height)
    } else {
        let ratio = ratio_height as f32 / ratio_width as f32;
        let height = (window_width as f32 * ratio) as u32;
        (window_width, height)
    };

    UVec2::new(width, height)
}

fn handle_window_resize_system(
    mut camera_query: Query<&mut Camera, With<MainCamera>>,
    mut resized_events: EventReader<WindowResized>,
) {
    for resized_event in resized_events.read() {
        let mut camera = camera_query.single_mut();
        let viewport = camera.viewport.as_mut().unwrap();

        let window_width = resized_event.width as u32;
        let window_height = resized_event.height as u32;
        let size = crop_aspect_ratio(4, 3, window_width, window_height);

        let window_size = UVec2::new(window_width, window_height);
        let gap = (window_size - size) / 2;

        viewport.physical_size = size;
        viewport.physical_position = gap;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system).add_systems(
            Update,
            (
                handle_window_resize_system,
                move_camera_down_system.run_if(input_pressed(KeyCode::KeyS)),
                move_camera_up_system.run_if(input_pressed(KeyCode::KeyW)),
                move_camera_left_system.run_if(input_pressed(KeyCode::KeyA)),
                move_camera_right_system.run_if(input_pressed(KeyCode::KeyD)),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crop_16x9_to_4x3() {
        let output = crop_aspect_ratio(4, 3, 1920, 1080);
        assert_eq!(output, UVec2::new(1440, 1080));
    }

    #[test]
    fn crop_16x9_to_1x1() {
        let output = crop_aspect_ratio(1, 1, 1920, 1080);
        assert_eq!(output, UVec2::new(1080, 1080));
    }

    #[test]
    fn crop_16x9_to_3x4() {
        let output = crop_aspect_ratio(3, 4, 1920, 1080);
        assert_eq!(output, UVec2::new(810, 1080));
    }

    #[test]
    fn crop_9x16_to_4x3() {
        let output = crop_aspect_ratio(4, 3, 1080, 1920);
        assert_eq!(output, UVec2::new(1080, 810));
    }

    #[test]
    fn crop_9x16_to_1x1() {
        let output = crop_aspect_ratio(1, 1, 1080, 1920);
        assert_eq!(output, UVec2::new(1080, 1080));
    }

    #[test]
    fn crop_9x16_to_3x4() {
        let output = crop_aspect_ratio(3, 4, 1080, 1920);
        assert_eq!(output, UVec2::new(1080, 1440));
    }
}
