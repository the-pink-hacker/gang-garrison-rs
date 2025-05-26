use std::collections::HashMap;

use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Foreground {
    #[serde(default = "Foreground::default_depth")]
    pub depth: i8,
    #[serde(default = "value_true")]
    pub fade: bool,
    #[serde(default = "value_f32_1")]
    pub opacity: f32,
    #[serde(default)]
    pub animationspeed: u8,
    #[serde(default)]
    pub trigger: u8,
    #[serde(default)]
    pub distance: u8,
    #[serde(default)]
    pub resource: String,
}

impl Foreground {
    #[inline]
    fn default_depth() -> i8 {
        -2
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveBox {
    #[serde(flatten)]
    pub transform: EntityTransform,
    // TODO: Confirm move box speed size
    #[serde(default = "MoveBox::default_speed")]
    pub speed: u8,
}

impl MoveBox {
    #[inline]
    fn default_speed() -> u8 {
        5
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityPosition {
    pub x: u32,
    pub y: u32,
}

impl From<EntityPosition> for Vec2 {
    fn from(value: EntityPosition) -> Self {
        Vec2::new(value.x as f32, value.y as f32)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityScale {
    #[serde(rename = "xscale", default = "value_f32_1")]
    pub x_scale: f32,
    #[serde(rename = "yscale", default = "value_f32_1")]
    pub y_scale: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityTransform {
    #[serde(flatten)]
    pub position: EntityPosition,
    #[serde(flatten)]
    pub scale: EntityScale,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MapEntity {
    #[serde(rename = "meta")]
    Meta(HashMap<String, serde_hjson::Value>),
    #[serde(rename = "spawnroom")]
    SpawnRoom(EntityTransform),
    #[serde(rename = "redspawn")]
    RedSpawn0(EntityPosition),
    #[serde(rename = "redspawn1")]
    RedSpawn1(EntityPosition),
    #[serde(rename = "readspawn2")]
    RedSpawn2(EntityPosition),
    #[serde(rename = "readspawn3")]
    RedSpawn3(EntityPosition),
    #[serde(rename = "readspawn4")]
    RedSpawn4(EntityPosition),
    #[serde(rename = "bluespawn")]
    BluSpawn0(EntityPosition),
    #[serde(rename = "bluespawn1")]
    BluSpawn1(EntityPosition),
    #[serde(rename = "bluespawn2")]
    BluSpawn2(EntityPosition),
    #[serde(rename = "bluespawn3")]
    BluSpawn3(EntityPosition),
    #[serde(rename = "bluespawn4")]
    BluSpawn4(EntityPosition),
    #[serde(rename = "redintel")]
    RedIntel(EntityPosition),
    #[serde(rename = "blueintel")]
    BluIntel(EntityPosition),
    #[serde(rename = "redteamgate")]
    RedTeamGate1(EntityTransform),
    #[serde(rename = "blueteamgate")]
    BluTeamGate1(EntityTransform),
    #[serde(rename = "redteamgate2")]
    RedTeamGate2(EntityTransform),
    #[serde(rename = "blueteamgate2")]
    BluTeamGate2(EntityTransform),
    #[serde(rename = "redintelgate")]
    RedIntelGate(EntityTransform),
    #[serde(rename = "blueintelgate")]
    BluIntelGate(EntityTransform),
    #[serde(rename = "redintelgate2")]
    RedIntelGate2(EntityTransform),
    #[serde(rename = "blueintelgate2")]
    BluIntelGate2(EntityTransform),
    #[serde(rename = "intelgatehorizontal")]
    IntelGateHorizontal(EntityTransform),
    #[serde(rename = "intelgatevertical")]
    IntelGateVertical(EntityTransform),
    #[serde(rename = "medCabinet")]
    MedicalCabinet {
        #[serde(flatten)]
        transform: EntityTransform,
        #[serde(default)]
        heal: bool,
        #[serde(default)]
        refill: bool,
        #[serde(default = "value_true")]
        uber: bool,
    },
    #[serde(rename = "killbox")]
    KillBox(EntityTransform),
    #[serde(rename = "pitfall")]
    PitFall(EntityTransform),
    #[serde(rename = "fragbox")]
    FragBox(EntityTransform),
    #[serde(rename = "playerwall")]
    PlayerWall(EntityTransform),
    #[serde(rename = "playerwall_horizontal")]
    PlayerWallHorizontal(EntityTransform),
    #[serde(rename = "bulletwall")]
    BulletWall {
        #[serde(flatten)]
        transform: EntityTransform,
        // TODO: Confirm bullet wall distance size
        #[serde(default = "value_i8_negative_1")]
        distance: i8,
    },
    #[serde(rename = "bulletwall_horizontal")]
    BulletWallHorizontal(EntityTransform),
    #[serde(rename = "leftdoor")]
    LeftDoor(EntityTransform),
    #[serde(rename = "rightdoor")]
    RightDoor(EntityTransform),
    #[serde(rename = "controlPoint1")]
    ControlPoint1(EntityPosition),
    #[serde(rename = "controlPoint2")]
    ControlPoint2(EntityPosition),
    #[serde(rename = "controlPoint3")]
    ControlPoint3(EntityPosition),
    #[serde(rename = "controlPoint4")]
    ControlPoint4(EntityPosition),
    #[serde(rename = "controlPoint5")]
    ControlPoint5(EntityPosition),
    #[serde(rename = "NextAreaO")]
    NextArea(EntityPosition),
    CapturePoint(EntityTransform),
    SetupGate(EntityTransform),
    ArenaControlPoint(EntityPosition),
    GeneratorRed(EntityPosition),
    GeneratorBlue(EntityPosition),
    MoveBoxUp(MoveBox),
    MoveBoxDown(MoveBox),
    MoveBoxLeft(MoveBox),
    MoveBoxRight(MoveBox),
    KothControlPoint(EntityPosition),
    KothRedControlPoint(EntityPosition),
    KothBlueControlPoint(EntityPosition),
    #[serde(rename = "dropdownPlatform")]
    DropDownPlatform {
        #[serde(flatten)]
        transform: EntityTransform,
        #[serde(rename = "reset_move_status", default = "value_u8_1")]
        reset_move_status: u8,
    },
    #[serde(rename = "foreground")]
    Foreground {
        #[serde(flatten)]
        transform: EntityTransform,
        #[serde(flatten)]
        foreground: Foreground,
    },
    #[serde(rename = "foreground_scale")]
    ForegroundScale {
        #[serde(flatten)]
        scale: EntityScale,
        #[serde(flatten)]
        foreground: Foreground,
    },
    #[serde(rename = "moving_platform")]
    MovingPlatform,
    #[serde(untagged)]
    Custom {
        #[serde(rename = "type")]
        entity_type: String,
        #[serde(flatten)]
        values: HashMap<String, serde_hjson::Value>,
    },
}

#[inline]
fn value_true() -> bool {
    true
}

#[inline]
fn value_i8_negative_1() -> i8 {
    -1
}

#[inline]
fn value_u8_1() -> u8 {
    1
}

#[inline]
fn value_f32_1() -> f32 {
    1.0
}
