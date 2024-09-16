use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    fn default_depth() -> i8 {
        -2
    }
}

#[derive(Debug, Deserialize)]
pub struct MoveBox {
    #[serde(flatten)]
    pub transform: Transform,
    // TODO: Confirm move box speed size
    #[serde(default = "MoveBox::default_speed")]
    pub speed: u8,
}

impl MoveBox {
    fn default_speed() -> u8 {
        5
    }
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Deserialize)]
pub struct Scale {
    #[serde(rename = "xscale", default = "value_f32_1")]
    pub x_scale: f32,
    #[serde(rename = "yscale", default = "value_f32_1")]
    pub y_scale: f32,
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    #[serde(flatten)]
    pub position: Position,
    #[serde(flatten)]
    pub scale: Scale,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MapEntity {
    #[serde(rename = "meta")]
    Meta {
        background: String,
        void: String,
    },
    #[serde(rename = "spawnroom")]
    SpawnRoom(Transform),
    #[serde(rename = "redspawn")]
    RedSpawn(Position),
    #[serde(rename = "redspawn1")]
    RedSpawn1(Position),
    #[serde(rename = "readspawn2")]
    RedSpawn2(Position),
    #[serde(rename = "readspawn3")]
    RedSpawn3(Position),
    #[serde(rename = "readspawn4")]
    RedSpawn4(Position),
    #[serde(rename = "bluespawn")]
    BluSpawn(Position),
    #[serde(rename = "bluespawn1")]
    BluSpawn1(Position),
    #[serde(rename = "bluespawn2")]
    BluSpawn2(Position),
    #[serde(rename = "bluespawn3")]
    BluSpawn3(Position),
    #[serde(rename = "bluespawn4")]
    BluSpawn4(Position),
    #[serde(rename = "redintel")]
    RedIntel(Position),
    #[serde(rename = "blueintel")]
    BluIntel(Position),
    #[serde(rename = "redteamgate")]
    RedTeamGate(Transform),
    #[serde(rename = "blueteamgate")]
    BluTeamGate(Transform),
    #[serde(rename = "redteamgate2")]
    RedTeamGate2(Transform),
    #[serde(rename = "blueteamgate2")]
    BluTeamGate2(Transform),
    #[serde(rename = "redintelgate")]
    RedIntelGate(Transform),
    #[serde(rename = "blueintelgate")]
    BluIntelGate(Transform),
    #[serde(rename = "redintelgate2")]
    RedIntelGate2(Transform),
    #[serde(rename = "blueintelgate2")]
    BluIntelGate2(Transform),
    #[serde(rename = "intelgatehorizontal")]
    IntelGateHorizontal(Transform),
    #[serde(rename = "intelgatevertical")]
    IntelGateVertical(Transform),
    #[serde(rename = "medCabinet")]
    MedicalCabinet {
        #[serde(flatten)]
        transform: Transform,
        #[serde(default)]
        heal: bool,
        #[serde(default)]
        refill: bool,
        #[serde(default = "value_true")]
        uber: bool,
    },
    #[serde(rename = "killbox")]
    KillBox(Transform),
    #[serde(rename = "pitfall")]
    PitFall(Transform),
    #[serde(rename = "fragbox")]
    FragBox(Transform),
    #[serde(rename = "playerwall")]
    PlayerWall(Transform),
    #[serde(rename = "playerwall_horizontal")]
    PlayerWallHorizontal(Transform),
    #[serde(rename = "bulletwall")]
    BulletWall {
        #[serde(flatten)]
        transform: Transform,
        // TODO: Confirm bullet wall distance size
        #[serde(default = "value_i8_negative_1")]
        distance: i8,
    },
    #[serde(rename = "bulletwall_horizontal")]
    BulletWallHorizontal(Transform),
    #[serde(rename = "leftdoor")]
    LeftDoor(Transform),
    #[serde(rename = "rightdoor")]
    RightDoor(Transform),
    #[serde(rename = "controlPoint1")]
    ControlPoint1(Position),
    #[serde(rename = "controlPoint2")]
    ControlPoint2(Position),
    #[serde(rename = "controlPoint3")]
    ControlPoint3(Position),
    #[serde(rename = "controlPoint4")]
    ControlPoint4(Position),
    #[serde(rename = "controlPoint5")]
    ControlPoint5(Position),
    #[serde(rename = "NextArea0")]
    NextArea(Position),
    CapturePoint(Transform),
    SetupGate(Transform),
    ArenaControlPoint(Position),
    GeneratorRed(Position),
    GeneratorBlue(Position),
    MoveBoxUp(MoveBox),
    MoveBoxDown(MoveBox),
    MoveBoxLeft(MoveBox),
    MoveBoxRight(MoveBox),
    KothControlPoint(Position),
    KothRedControlPoint(Position),
    KothBlueControlPoint(Position),
    #[serde(rename = "dropdownPlatform")]
    DropDownPlatform {
        #[serde(flatten)]
        transform: Transform,
        #[serde(rename = "reset_move_status", default = "value_u8_1")]
        reset_move_status: u8,
    },
    #[serde(rename = "foreground")]
    Foreground {
        #[serde(flatten)]
        transform: Transform,
        #[serde(flatten)]
        foreground: Foreground,
    },
    #[serde(rename = "foreground_scale")]
    ForegroundScale {
        #[serde(flatten)]
        scale: Scale,
        #[serde(flatten)]
        foreground: Foreground,
    },
    #[serde(rename = "moving_platform")]
    MovingPlatform,
}

fn value_true() -> bool {
    true
}

fn value_i8_negative_1() -> i8 {
    -1
}

fn value_u8_1() -> u8 {
    1
}

fn value_f32_1() -> f32 {
    1.0
}
