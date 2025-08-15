use std::{pin::Pin, time::Duration};

use crate::prelude::*;

pub trait GamemodeState: Send + Sync {
    fn tick<'a>(
        &'a mut self,
        world: &'a dyn World,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommonError>> + 'a + Send>>;
}

#[derive(Debug, Default)]
pub struct CaptureTheFlagState {
    pub match_timer: HudMatchTimer,
}

impl GamemodeState for CaptureTheFlagState {
    fn tick<'a>(
        &'a mut self,
        world: &'a dyn World,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommonError>> + 'a + Send>> {
        Box::pin(async {
            println!("DELTA: {}", world.delta_tick());
            self.match_timer.left = self
                .match_timer
                .left
                .saturating_sub(Duration::from_secs_f32(world.delta_tick()));
            Ok(())
        })
    }
}
