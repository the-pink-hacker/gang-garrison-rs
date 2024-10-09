use bevy::{ecs::system::EntityCommands, prelude::*};
use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub use scout::ClassScout;
pub use soldier::ClassSoldier;

use crate::physics::collider_rectangle;

use super::PositionShift;

mod scout;
mod soldier;

#[derive(
    Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence,
)]
#[repr(u8)]
pub enum ClassGeneric {
    #[default]
    Scout,
    Soldier,
    Sniper,
    Demoman,
    Medic,
    Engineer,
    Heavy,
    Spy,
    Pyro,
    Quote,
}

impl ClassGeneric {
    pub fn add_class_components(self, commands: &mut EntityCommands) {
        let method = match self {
            Self::Scout => ClassScout::add_class_components,
            Self::Soldier => ClassSoldier::add_class_components,
            _ => todo!(),
        };
        commands.insert(self);
        method(commands);
    }

    pub fn change_class(class: ClassGeneric, commands: &mut EntityCommands) {
        let commands = commands.insert(class).remove::<ClassesBundle>();
        class.add_class_components(commands);
    }
}

#[derive(Bundle)]
struct ClassesBundle {
    scout: ClassScout,
    soldier: ClassSoldier,
}

trait Class: Component {
    const POSITION_SHIFT: Vec2;
    const SIZE: Vec2;

    fn add_class_components(commands: &mut EntityCommands) {
        let commands = commands.insert((
            PositionShift::from(Vec2::new(1.0, 8.0)),
            collider_rectangle((
                Self::POSITION_SHIFT.x - 1.0,
                Self::POSITION_SHIFT.y - 8.0,
                Self::SIZE.x,
                Self::SIZE.y,
            )),
        ));
        Self::add_additional_components(commands);
    }

    fn add_additional_components(commands: &mut EntityCommands);
}
