use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CommonPhysicsPlugin;

impl Plugin for CommonPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    }
}
