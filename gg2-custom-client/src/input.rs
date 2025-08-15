pub mod bind;
pub mod code;
pub mod device;

use std::{ops::Deref, pin::Pin, sync::Arc};

use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct InputAxisResult(f32);

impl From<f32> for InputAxisResult {
    fn from(value: f32) -> Self {
        assert!((-1.0..=1.0).contains(&value));

        Self(value)
    }
}

impl Deref for InputAxisResult {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct InputButtonResult(f32);

const BUTTON_PRESSED: InputButtonResult = InputButtonResult(1.0);
const BUTTON_UNPRESSED: InputButtonResult = InputButtonResult(0.0);

impl InputButtonResult {
    pub fn is_pressed(&self) -> bool {
        self.0 > 0.0
    }
}

impl From<f32> for InputButtonResult {
    #[inline]
    fn from(value: f32) -> Self {
        assert!((0.0..=1.0).contains(&value));

        Self(value)
    }
}

impl Deref for InputButtonResult {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait InputPoll {
    fn poll_axis_bind(
        &self,
        bind: &InputAxisBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputAxisResult>>>;

    fn poll_button_bind(
        &self,
        bind: &InputButtonBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputButtonResult>>>;
}

#[derive(Debug)]
pub struct InputState {
    current_device: Arc<dyn InputDevice>,
}

impl InputState {
    pub fn new(device: Arc<dyn InputDevice>) -> Self {
        Self {
            current_device: device,
        }
    }

    pub fn register_device(&mut self, device: Arc<dyn InputDevice>) {
        trace!("Input device has been swapped to {:?}", device.get_name());
        self.current_device = device;
    }

    #[inline]
    pub async fn poll_axis_bind(
        &self,
        bind: &InputAxisBind,
        world: &'static ClientWorld,
    ) -> Option<InputAxisResult> {
        self.current_device.poll_axis_bind(bind, world).await
    }

    #[inline]
    pub async fn poll_button_bind(
        &self,
        bind: &InputButtonBind,
        world: &'static ClientWorld,
    ) -> Option<InputButtonResult> {
        self.current_device.poll_button_bind(bind, world).await
    }
}
