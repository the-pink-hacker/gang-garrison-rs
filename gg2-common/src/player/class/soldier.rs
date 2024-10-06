use bevy::{ecs::system::EntityCommands, prelude::*};

use super::Class;

#[derive(Component)]
pub struct ClassSoldier;

impl Class for ClassSoldier {
    const POSITION_SHIFT: Vec2 = Vec2::new(-6.0, 10.0);
    const SIZE: Vec2 = Vec2::new(12.0, 31.0);

    fn add_additional_components(commands: &mut EntityCommands) {
        commands.insert(Self);
    }
}
