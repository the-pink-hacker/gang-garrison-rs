use std::{collections::HashMap, pin::Pin, sync::Arc, time::Duration};

use dyn_future::DynFuture;
use gilrs::{Filter, GamepadId, Gilrs, GilrsBuilder, ev::filter::Jitter};

use crate::prelude::*;

const GILRS_POLL_INTERVAL: f32 = crate::game::GAME_LOOP_INTERVAL;
const JITTER: Jitter = Jitter { threshold: 0.01 };

pub struct GilrsWatcher {
    gilrs: Gilrs,
    world: &'static ClientWorld,
}

impl GilrsWatcher {
    pub fn new(world: &'static ClientWorld) -> Self {
        Self {
            gilrs: GilrsBuilder::new()
                .set_update_state(false)
                .build()
                .expect("Failed to setup gilrs"),
            world,
        }
    }

    /// Never yeilds
    pub async fn listen(mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs_f32(GILRS_POLL_INTERVAL));

        loop {
            interval.tick().await;

            while let Some(event) = self.next_event() {
                trace!("{event:#?}");

                self.world
                    .client_game_channel()
                    .send(ClientGameMessage::GilrsEvent(event))
                    .expect("Failed to send gilrs event.");
            }
        }
    }

    fn next_event(&mut self) -> Option<gilrs::Event> {
        self.gilrs
            .next_event()
            .filter_ev(&JITTER, &mut self.gilrs)
            .filter_ev(&gilrs::ev::filter::deadzone, &mut self.gilrs)
    }
}

impl ClientGame {
    pub async fn handle_gilrs_event(&self, event: gilrs::Event) -> Result<(), ClientError> {
        let mut state = self.world.gilrs_input_state().write().await;

        use gilrs::EventType;

        match event.event {
            EventType::Connected => state.connect_gamepad(event.id),
            EventType::Disconnected => state.disconnect_gamepad(event.id),
            EventType::AxisChanged(axis, value, _) => state.update_axis(event.id, axis, value),
            EventType::ButtonChanged(button, value, _) => {
                self.world
                    .input_state()
                    .write()
                    .await
                    .register_device(Arc::new(GilrsInputDevice {
                        name: format!("Gilrs Controller #{}", event.id),
                        gamepad: event.id,
                    }));
                state.update_button(event.id, button, value);
            }
            EventType::ButtonPressed(button, _) => debug!("Pressed gamepad button: {button:?}"),
            EventType::ButtonReleased(button, _) => debug!("Released gamepad button: {button:?}"),
            _ => (),
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct GilrsGamepadState {
    axis: HashMap<InputAxisCode, InputAxisResult>,
    buttons: HashMap<InputButtonCode, InputButtonResult>,
}

#[derive(Debug, Default)]
pub struct GilrsInputState {
    gamepads: HashMap<gilrs::GamepadId, GilrsGamepadState>,
}

impl GilrsInputState {
    fn connect_gamepad(&mut self, id: GamepadId) {
        self.gamepads.entry(id).or_default();
    }

    fn disconnect_gamepad(&mut self, id: GamepadId) {
        self.gamepads.remove_entry(&id);
    }

    #[inline]
    fn update_axis(
        &mut self,
        id: GamepadId,
        axis: impl Into<InputAxisCode>,
        value: impl Into<InputAxisResult>,
    ) {
        self._update_axis(id, axis.into(), value.into());
    }

    fn _update_axis(&mut self, id: GamepadId, axis: InputAxisCode, value: InputAxisResult) {
        self.gamepads
            .entry(id)
            .or_default()
            .axis
            .entry(axis)
            .insert_entry(value);
    }

    #[inline]
    fn update_button(
        &mut self,
        id: GamepadId,
        button: impl Into<InputButtonCode>,
        value: impl Into<InputButtonResult>,
    ) {
        self._update_button(id, button.into(), value.into());
    }

    fn _update_button(&mut self, id: GamepadId, button: InputButtonCode, value: InputButtonResult) {
        self.gamepads
            .entry(id)
            .or_default()
            .buttons
            .entry(button)
            .insert_entry(value);
    }
}

pub struct GilrsInputDevice {
    name: String,
    gamepad: gilrs::GamepadId,
}

impl InputDevice for GilrsInputDevice {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl InputPoll for GilrsInputDevice {
    fn poll_axis_bind(
        &self,
        bind: &InputAxisBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputAxisResult>>> {
        let gamepad = self.gamepad;
        let bind = bind.clone();
        DynFuture::new(async move {
            world
                .gilrs_input_state()
                .read()
                .await
                .gamepads
                .get(&gamepad)
                .and_then(|state| {
                    bind.into_iter()
                        .flat_map(|axis| state.axis.get(&axis))
                        .next()
                        .cloned()
                })
        })
    }

    fn poll_button_bind(
        &self,
        bind: &InputButtonBind,
        world: &'static ClientWorld,
    ) -> Pin<DynFuture<Option<InputButtonResult>>> {
        let gamepad = self.gamepad;
        let bind = bind.clone();
        DynFuture::new(async move {
            world
                .gilrs_input_state()
                .read()
                .await
                .gamepads
                .get(&gamepad)
                .and_then(|state| {
                    bind.into_iter()
                        .flat_map(|button| {
                            if let Some(result) = state.buttons.get(&button)
                                && result.is_pressed()
                            {
                                Some(result)
                            } else {
                                None
                            }
                        })
                        .next()
                        .cloned()
                })
        })
    }
}
