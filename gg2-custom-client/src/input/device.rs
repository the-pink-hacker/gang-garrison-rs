pub mod gilrs;
pub mod winit;

use crate::prelude::*;

pub trait InputDevice: InputPoll + Send + Sync {
    /// The name of the device.
    fn get_name(&self) -> &str;
}

impl std::fmt::Debug for dyn InputDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}
