use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct RawIntel {
    pub position: Vec2,
    pub recharge_time: Duration,
}
