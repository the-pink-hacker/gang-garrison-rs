use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RawIntel {
    pub amount: u16,
    pub x: f32,
    pub y: f32,
    pub recharge_time: Duration,
}
