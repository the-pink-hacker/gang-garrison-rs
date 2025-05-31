use std::time::Duration;

use glam::Vec2;

#[derive(Debug, Clone)]
pub struct RawIntel {
    pub position: Vec2,
    pub recharge_time: Duration,
}
