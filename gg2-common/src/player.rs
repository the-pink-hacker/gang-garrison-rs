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

impl KeyState {
    const UP_MASK: u8 = 1 << 7;
    const DOWN_MASK: u8 = 1 << 1;
    const LEFT_MASK: u8 = 1 << 6;
    const RIGHT_MASK: u8 = 1 << 5;
}

impl KeyState {
    #[inline]
    #[must_use]
    pub const fn up(&self) -> bool {
        self.0 & Self::UP_MASK != 0
    }

    #[inline]
    #[must_use]
    pub const fn down(&self) -> bool {
        self.0 & Self::DOWN_MASK != 0
    }

    #[inline]
    #[must_use]
    pub const fn left(&self) -> bool {
        self.0 & Self::LEFT_MASK != 0
    }

    #[inline]
    #[must_use]
    pub const fn right(&self) -> bool {
        self.0 & Self::RIGHT_MASK != 0
    }

    #[inline]
    pub const fn set_up(&mut self, state: bool) {
        if state {
            self.0 |= Self::UP_MASK;
        } else {
            self.0 &= Self::UP_MASK ^ u8::MAX;
        }
    }

    #[inline]
    pub const fn set_down(&mut self, state: bool) {
        if state {
            self.0 |= Self::DOWN_MASK;
        } else {
            self.0 &= Self::DOWN_MASK ^ u8::MAX;
        }
    }

    #[inline]
    pub const fn set_left(&mut self, state: bool) {
        if state {
            self.0 |= Self::LEFT_MASK;
        } else {
            self.0 &= Self::LEFT_MASK ^ u8::MAX;
        }
    }

    #[inline]
    pub const fn set_right(&mut self, state: bool) {
        if state {
            self.0 |= Self::RIGHT_MASK;
        } else {
            self.0 &= Self::RIGHT_MASK ^ u8::MAX;
        }
    }
}

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
