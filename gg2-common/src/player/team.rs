use bevy::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Team {
    Red,
    Blu,
    #[default]
    Spectator,
}

#[derive(Debug, Default, Clone, Copy, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum TeamChoice {
    Red,
    Blu,
    Spectator,
    #[default]
    Any,
}
