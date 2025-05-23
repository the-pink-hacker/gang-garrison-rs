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
pub struct KeyState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl KeyState {
    const UP_MASK: u8 = 1 << 7;
    const DOWN_MASK: u8 = 1 << 1;
    const LEFT_MASK: u8 = 1 << 6;
    const RIGHT_MASK: u8 = 1 << 5;
}

impl From<u8> for KeyState {
    fn from(value: u8) -> Self {
        let up = value & Self::UP_MASK != 0;
        let down = value & Self::DOWN_MASK != 0;
        let left = value & Self::LEFT_MASK != 0;
        let right = value & Self::RIGHT_MASK != 0;

        Self {
            up,
            down,
            left,
            right,
        }
    }
}

impl From<KeyState> for u8 {
    fn from(value: KeyState) -> Self {
        let mut output = 0;

        if value.up {
            output |= KeyState::UP_MASK;
        }
        if value.down {
            output |= KeyState::DOWN_MASK;
        }
        if value.left {
            output |= KeyState::LEFT_MASK;
        }
        if value.right {
            output |= KeyState::RIGHT_MASK;
        }

        output
    }
}

#[derive(Debug, Default, Clone)]
pub struct RawInput {
    pub key_state: KeyState,
    pub net_aim_direction: u16,
    pub aim_distance: f32,
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
