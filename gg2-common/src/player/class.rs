use std::fmt::Display;

use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(
    Debug, Default, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
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
        write!(f, "{self:?}")
    }
}
