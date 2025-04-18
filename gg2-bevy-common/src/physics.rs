use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CommonPhysicsPlugin;

impl Plugin for CommonPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

pub fn collider_rectangle(bounds: impl Into<(f32, f32, f32, f32)>) -> Collider {
    let (x, y, width, height) = bounds.into();
    Collider::trimesh(
        vec![
            Vec2::new(x, y),
            Vec2::new(x + width, y),
            Vec2::new(x, y - height),
            Vec2::new(x + width, y - height),
        ],
        vec![[0, 1, 2], [1, 2, 3]],
    )
    .expect("Failed to generate rectangle collider")
}
