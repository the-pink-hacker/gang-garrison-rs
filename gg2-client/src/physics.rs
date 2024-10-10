use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use gg2_common::physics::*;

pub struct ClientPhysicsPlugin;

impl Plugin for ClientPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CommonPhysicsPlugin, RapierDebugRenderPlugin::default()));
    }
}
