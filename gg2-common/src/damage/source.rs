use bevy::prelude::*;
use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    Debug, Default, Component, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence,
)]
#[repr(u8)]
pub enum DamageSource {
    Needlegun,
    Rifle,
    RifleCharged,
    Minegun,
    Minigun,
    Flamethrower,
    Scattergun,
    Shotgun,
    RocketLauncher,
    Revolver,
    SentryTurret,
    Blade,
    Bubble,
    ReflectedRocket,
    ReflectedSticky,
    Knife,
    Backstab,
    Flare,
    ReflectedFlare,
    KillBox,
    FragBox,
    Pitfall,
    FinishedOff,
    FinishedOffGib,
    #[default]
    BidFarewell,
    GeneratorExplosion,
}
