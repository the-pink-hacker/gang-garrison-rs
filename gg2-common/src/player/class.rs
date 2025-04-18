use std::fmt::Display;

use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Default, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence)]
#[cfg_attr(feature = "bevy", derive(bevy::ecs::component::Component))]
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

impl Display for ClassGeneric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
