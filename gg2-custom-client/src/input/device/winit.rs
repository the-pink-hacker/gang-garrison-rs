use std::{collections::HashMap, pin::Pin};

use dyn_future::DynFuture;
use uuid::Uuid;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, WindowEvent},
    keyboard::PhysicalKey,
};

use crate::{
    input::{BUTTON_PRESSED, BUTTON_UNPRESSED},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct WinitInputState {
    mouse_position: Vec2,
    buttons: HashMap<InputButtonCode, bool>,
}

impl WinitInputState {
    #[inline]
    fn set_button(&mut self, button: impl Into<InputButtonCode>, state: &ElementState) {
        self._set_button(button.into(), state);
    }

    fn _set_button(&mut self, button: InputButtonCode, state: &ElementState) {
        self.buttons.entry(button).insert_entry(state.is_pressed());
    }

    fn set_mouse_position(&mut self, position: &PhysicalPosition<f64>) {
        self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
    }

    fn get_button(&self, code: InputButtonCode) -> Option<InputButtonResult> {
        let pressed = *self.buttons.get(&code)?;

        Some(if pressed {
            BUTTON_PRESSED
        } else {
            BUTTON_UNPRESSED
        })
    }
}

impl crate::render::State {
    pub async fn handle_window_event_input(&self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.world
                    .winit_input_state()
                    .write()
                    .await
                    .set_mouse_position(position);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.world
                    .winit_input_state()
                    .write()
                    .await
                    .set_button(*button, state);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => match event.physical_key {
                PhysicalKey::Code(button) => self
                    .world
                    .winit_input_state()
                    .write()
                    .await
                    .set_button(button, &event.state),
                PhysicalKey::Unidentified(code) => {
                    warn!("Native key codes are unsupported: {code:?}")
                }
            },
            _ => (),
        }
    }
}

pub struct WinitInputDevice;

impl InputDevice for WinitInputDevice {
    fn get_name(&self) -> &str {
        "Winit Input"
    }

    fn get_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }
}

impl InputPoll for WinitInputDevice {
    fn poll_button_bind(
        &self,
        bind: &InputButtonBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputButtonResult>>> {
        let bind = bind.clone();
        DynFuture::new(async move {
            let mut value_present = false;
            let state = world.winit_input_state().read().await;
            for key in bind {
                if let Some(result) = state.get_button(key) {
                    value_present = true;

                    if result.is_pressed() {
                        return Some(result);
                    }
                }
            }

            if value_present {
                Some(InputButtonResult::default())
            } else {
                None
            }
        })
    }
}
