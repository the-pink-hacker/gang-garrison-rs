use std::fmt::Display;

use glam::Vec2;

use crate::error::{CommonError as Error, Result};

pub mod class;
pub mod team;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "u8", try_from = "u8"))]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn from_u8(value: u8) -> Option<PlayerId> {
        match value {
            0..255 => Some(Self(value)),
            255 => None,
        }
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl From<PlayerId> for u8 {
    fn from(value: PlayerId) -> Self {
        value.0
    }
}

impl From<PlayerId> for usize {
    fn from(value: PlayerId) -> Self {
        value.0 as usize
    }
}

impl TryFrom<u8> for PlayerId {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Self::from_u8(value).ok_or(Error::PlayerIdInvalid)
    }
}

impl TryFrom<usize> for PlayerId {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self> {
        u8::try_from(value)
            .map_err(Error::PlayerIdOutOfBounds)
            .and_then(Self::try_from)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyState(u8);

macro_rules! key_state {
    ($(($name: ident, $offset: literal)),+$(,)?) => {
        impl KeyState {
            $(
                #[inline]
                #[must_use]
                pub const fn $name(&self) -> bool {
                    self.0 & (1 << $offset) != 0
                }

                #[inline]
                pub const fn ${concat(set_, $name)}(&mut self, state: bool) {
                    let offset = 1 << $offset;
                    if state {
                        self.0 |= offset;
                    } else {
                        self.0 &= offset ^ u8::MAX;
                    }
                }
            )+
        }
    };
}

key_state![
    (taunt, 1),
    (down, 2),
    (secondary, 3),
    (primary, 4),
    (right, 5),
    (left, 6),
    (up, 7),
];

impl From<u8> for KeyState {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<KeyState> for u8 {
    fn from(value: KeyState) -> Self {
        value.0
    }
}

#[derive(Debug, Default, Clone)]
pub struct RawInput {
    pub key_state: KeyState,
    pub aim_direction: u16,
    pub aim_distance: f32,
}

impl RawInput {
    pub fn looking_left(&self) -> bool {
        let quater_rotation = u16::MAX / 4;
        let aim_rotation = self.aim_direction.wrapping_sub(quater_rotation);
        aim_rotation <= (u16::MAX / 2) + 2
    }
}

#[derive(Debug, Default, Clone)]
pub struct RawPlayerInfo {
    pub translation: Vec2,
    pub velocity: Vec2,
    pub health: u8,
    pub ammo_count: u8,
    // TODO: Add move status
    pub move_status: u8,
}

#[derive(Debug, Clone)]
pub struct RawAdditionalPlayerInfo {}
