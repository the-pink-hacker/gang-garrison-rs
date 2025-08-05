use std::{collections::HashMap, pin::Pin, sync::Arc};

use dyn_future::DynFuture;
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

    fn set_mouse_position(&mut self, position: &PhysicalPosition<f64>, game_size: UVec2) {
        self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
        let half_game = (game_size / 2).as_vec2();
        self.mouse_position -= half_game;
        self.mouse_position = self.mouse_position.clamp(-half_game, half_game);
        self.mouse_position /= game_size.as_vec2();
        self.mouse_position *= Vec2::new(
            (crate::render::GAME_WIDTH / 4) as f32,
            (crate::render::GAME_HEIGHT / 4) as f32,
        );
    }

    #[inline]
    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
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
                    .set_mouse_position(position, self.game_size);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.world.register_winit_device().await;

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
            } => {
                self.world.register_winit_device().await;

                match event.physical_key {
                    PhysicalKey::Code(button) => self
                        .world
                        .winit_input_state()
                        .write()
                        .await
                        .set_button(button, &event.state),
                    PhysicalKey::Unidentified(code) => {
                        warn!("Native key codes are unsupported: {code:?}")
                    }
                }
            }
            _ => (),
        }
    }
}

impl ClientWorld {
    async fn register_winit_device(&self) {
        self.input_state()
            .write()
            .await
            .register_device(Arc::clone(self.winit_input_device()));
    }
}

#[derive(Default)]
pub struct WinitInputDevice {}

impl InputDevice for WinitInputDevice {
    fn get_name(&self) -> &str {
        "Winit Input"
    }
}

impl InputPoll for WinitInputDevice {
    fn poll_axis_bind(
        &self,
        _bind: &InputAxisBind,
        _world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputAxisResult>>> {
        DynFuture::new(async { None })
    }

    fn poll_button_bind(
        &self,
        bind: &InputButtonBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputButtonResult>>> {
        let bind = bind.clone();
        DynFuture::new(async move {
            let state = world.winit_input_state().read().await;
            bind.into_iter()
                .flat_map(|key| {
                    if let Some(result) = state.get_button(key)
                        && result.is_pressed()
                    {
                        Some(result)
                    } else {
                        None
                    }
                })
                .next()
        })
    }
}
