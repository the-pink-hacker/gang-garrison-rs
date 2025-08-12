use crate::prelude::*;

pub trait GamemodeState {}

#[derive(Debug, Default)]
pub struct CaptureTheFlagState {
    pub match_timer: HudMatchTimer,
}

impl GamemodeState for CaptureTheFlagState {}
