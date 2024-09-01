use bevy::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Component)]
pub struct Player {
    pub name: String,
}

#[derive(Debug, Component, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Team {
    Red,
    Blu,
    Spectator,
    Any,
}

#[derive(Debug, Component, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Class {
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
