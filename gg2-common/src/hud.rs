use std::time::Duration;

use crate::game::{control_point::RawControlPoint, generator::RawGenerator};

#[derive(Debug, Clone)]
pub struct HudMatchTimer {
    pub start: Duration,
    pub current: Duration,
}

#[derive(Debug, Clone)]
pub struct HudKothTimer {
    pub capture_unlock: Duration,
    pub red_timer: Duration,
    pub blu_timer: Duration,
}

#[derive(Debug, Clone)]
pub struct GamemodeHudArenaFull {
    pub red_wins: u8,
    pub blu_wins: u8,
    pub state: u8,
    pub winners: u8,
    pub end_count: u16,
}

#[derive(Debug, Clone)]
pub enum GamemodeHud {
    Arena {
        full_update: Option<GamemodeHudArenaFull>,
        match_timer: HudMatchTimer,
        control_point_unlock: Duration,
        round_start: u8,
        control_point: RawControlPoint,
    },
    CaptureTheFlag {
        match_timer: HudMatchTimer,
    },
    ControlPoint {
        match_timer: HudMatchTimer,
        setup_timer: Duration,
        control_points: Vec<RawControlPoint>,
    },
    KingOfTheHill {
        timer: HudKothTimer,
        control_point: RawControlPoint,
    },
    DualKingOfTheHill {
        timer: HudKothTimer,
        red_control_point: RawControlPoint,
        blu_control_point: RawControlPoint,
    },
    Generator {
        match_timer: HudMatchTimer,
        blu_generator: RawGenerator,
        red_generator: RawGenerator,
    },
    Invasion {
        match_timer: HudMatchTimer,
        setup_timer: Duration,
    },
    TeamDeathmatch {
        match_timer: HudMatchTimer,
        kill_limit: u16,
    },
}
