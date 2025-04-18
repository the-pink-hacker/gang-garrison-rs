use bevy::{ecs::system::EntityCommands, prelude::*};

use super::Class;

#[derive(Component)]
pub struct ClassScout;

impl Class for ClassScout {
    const POSITION_SHIFT: Vec2 = Vec2::new(-6.0, 10.0);
    const SIZE: Vec2 = Vec2::new(12.0, 33.0);

    fn add_additional_components(commands: &mut EntityCommands) {
        commands.insert(Self);
    }
}
