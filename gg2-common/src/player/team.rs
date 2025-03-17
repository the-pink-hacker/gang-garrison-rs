use std::fmt::Display;

use bevy::prelude::*;
use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::{Error, Result};

#[derive(
    Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence,
)]
#[repr(u8)]
pub enum Team {
    Red,
    Blu,
    #[default]
    Spectator,
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(
    Debug, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence,
)]
#[repr(u8)]
pub enum TeamSpawnable {
    Red,
    Blu,
}

impl TryFrom<&Team> for TeamSpawnable {
    type Error = Error;

    fn try_from(value: &Team) -> Result<Self> {
        match value {
            Team::Red => Ok(Self::Red),
            Team::Blu => Ok(Self::Blu),
            Team::Spectator => Err(Error::SpawnSpectator),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence)]
#[repr(u8)]
pub enum TeamChoice {
    Red,
    Blu,
    Spectator,
    #[default]
    Any,
}
