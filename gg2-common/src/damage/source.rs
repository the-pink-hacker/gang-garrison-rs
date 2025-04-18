use enum_iterator::Sequence;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Default, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq, Sequence)]
#[cfg_attr(feature = "bevy", derive(bevy::ecs::component::Component))]
#[repr(u8)]
pub enum DamageSource {
    // TODO: Figure out if there's an index 0 damage source
    None,
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
