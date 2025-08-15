use std::pin::Pin;

use crate::prelude::*;

pub trait World: Send + Sync {
    fn players(&self) -> &RwLock<dyn Players>;

    /// The number of seconds since the last game tick.
    fn delta_tick(&self) -> f32;

    fn set_delta_tick(&self, seconds: f32);

    #[allow(clippy::type_complexity)]
    fn with_gamemode_state_mut(
        &self,
        function: Box<
            dyn FnOnce(
                    Option<&mut dyn GamemodeState>,
                )
                    -> Pin<Box<dyn Future<Output = Result<(), CommonError>> + '_ + Send>>
                + Send,
        >,
    ) -> Pin<Box<dyn Future<Output = Result<(), CommonError>> + '_ + Send>>;
}
