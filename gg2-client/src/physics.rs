use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::*;
use gg2_common::physics::*;

fn debug_physics_render_toggle_system(mut debug_render_context: ResMut<DebugRenderContext>) {
    let enabled = !debug_render_context.enabled;
    println!("Debug Physics Renderer Enabled: {}", enabled);
    debug_render_context.enabled = enabled;
}

pub struct ClientPhysicsPlugin;

impl Plugin for ClientPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CommonPhysicsPlugin, RapierDebugRenderPlugin::default()))
            .add_systems(
                Update,
                debug_physics_render_toggle_system.run_if(input_just_pressed(KeyCode::F3)),
            );
    }
}
