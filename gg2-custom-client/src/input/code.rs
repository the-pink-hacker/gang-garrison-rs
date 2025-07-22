use gilrs::{Axis, Button};
use serde::{Deserialize, Serialize};
use winit::{event::MouseButton, keyboard::KeyCode};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputButtonCode {
    /// Winit keyboard codes
    Keyboard(KeyCode),
    Gamepad(Button),
    // TODO: Fix mouse prefix on deserialization
    //#[serde(with = "prefix_mouse")]
    Mouse(MouseButton),
}

//with_prefix!(prefix_mouse "Mouse");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InputAxisCode {
    Gamepad(Axis),
}

impl From<KeyCode> for InputButtonCode {
    #[inline]
    fn from(value: KeyCode) -> Self {
        Self::Keyboard(value)
    }
}

impl From<Button> for InputButtonCode {
    #[inline]
    fn from(value: Button) -> Self {
        Self::Gamepad(value)
    }
}

impl From<MouseButton> for InputButtonCode {
    #[inline]
    fn from(value: MouseButton) -> Self {
        Self::Mouse(value)
    }
}

impl From<Axis> for InputAxisCode {
    #[inline]
    fn from(value: Axis) -> Self {
        Self::Gamepad(value)
    }
}
