use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use gilrs::{Axis, Button};
use serde::{Deserialize, Serialize};
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::prelude::*;

mod error;
mod io;

#[derive(Debug)]
pub struct ClientConfig {
    values: ClientConfigRoot,
    /// The path where the config is stored
    path: PathBuf,
}

impl Deref for ClientConfig {
    type Target = ClientConfigRoot;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for ClientConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigRoot {
    pub networking: ClientConfigNetworking,
    pub game: ClientConfigGame,
    pub controls: ClientConfigControls,
    pub assets: ClientConfigAssets,
    pub debug: ClientConfigDebug,

    /// Doesn't override unknown values
    #[serde(flatten)]
    _extra: toml::Table,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigGame {
    pub player_name: GGStringShort,
}

impl Default for ClientConfigGame {
    fn default() -> Self {
        Self {
            player_name: "Rust Player"
                .to_string()
                .try_into()
                .expect("Failed to create default player name"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigDebug {
    pub gui: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigNetworking {
    pub default_server_address: String,
}

impl Default for ClientConfigNetworking {
    fn default() -> Self {
        Self {
            default_server_address: format!("127.0.0.1:{}", gg2_common::networking::DEFAULT_PORT),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigControls {
    pub up: InputButtonBind,
    // Wait is this used?
    pub down: InputButtonBind,
    pub left: InputButtonBind,
    pub right: InputButtonBind,
    pub move_vertical: InputAxisBind,
    pub move_horizontal: InputAxisBind,
    pub look_vertical: InputAxisBind,
    pub look_horizontal: InputAxisBind,
    pub fire: InputButtonBind,
    pub special: InputButtonBind,
    pub taunt: InputButtonBind,
    pub call_healer: InputButtonBind,
    pub drop: InputButtonBind,
    pub emote_main: InputButtonBind,
    pub emote_gameplay: InputButtonBind,
    pub emote_direction: InputButtonBind,
    pub debug_menu: InputButtonBind,
    pub menu: InputButtonBind,
    pub show_scores: InputButtonBind,
    pub select_team: InputButtonBind,
    pub select_class: InputButtonBind,
    pub enter: InputButtonBind,
    pub exit: InputButtonBind,
}

impl Default for ClientConfigControls {
    fn default() -> Self {
        Self {
            up: vec![KeyCode::KeyW.into(), KeyCode::ArrowUp.into()].into(),
            down: vec![KeyCode::KeyS.into(), KeyCode::ArrowDown.into()].into(),
            left: vec![KeyCode::KeyA.into(), KeyCode::ArrowLeft.into()].into(),
            right: vec![KeyCode::KeyD.into(), KeyCode::ArrowRight.into()].into(),
            move_vertical: vec![Axis::LeftStickY.into(), Axis::DPadY.into()].into(),
            move_horizontal: vec![Axis::LeftStickX.into(), Axis::DPadX.into()].into(),
            look_vertical: vec![Axis::RightStickY.into()].into(),
            look_horizontal: vec![Axis::RightStickX.into()].into(),
            fire: vec![MouseButton::Left.into(), Button::RightTrigger.into()].into(),
            special: vec![MouseButton::Right.into(), Button::LeftTrigger2.into()].into(),
            taunt: vec![KeyCode::KeyF.into(), Button::LeftThumb.into()].into(),
            call_healer: vec![KeyCode::KeyE.into(), Button::RightTrigger2.into()].into(),
            drop: vec![KeyCode::KeyB.into(), Button::East.into()].into(),
            emote_main: vec![KeyCode::KeyZ.into()].into(),
            emote_gameplay: vec![KeyCode::KeyX.into()].into(),
            emote_direction: vec![KeyCode::KeyC.into()].into(),
            debug_menu: vec![KeyCode::F3.into()].into(),
            menu: vec![KeyCode::Escape.into()].into(),
            show_scores: vec![KeyCode::ShiftLeft.into()].into(),
            select_team: vec![KeyCode::KeyN.into(), Button::West.into()].into(),
            select_class: vec![KeyCode::KeyM.into(), Button::North.into()].into(),
            enter: vec![KeyCode::Enter.into(), Button::South.into()].into(),
            exit: vec![KeyCode::Escape.into(), Button::East.into()].into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfigAssets {
    pub enabled_packs: Vec<String>,
}
