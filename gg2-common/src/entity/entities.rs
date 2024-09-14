use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MapEntity {
    #[serde(rename = "meta")]
    Meta,
    #[serde(rename = "spawnroom")]
    SpawnRoom,
    #[serde(rename = "redspawn")]
    RedSpawn,
    #[serde(rename = "redspawn1")]
    RedSpawn1,
    #[serde(rename = "readspawn2")]
    RedSpawn2,
    #[serde(rename = "readspawn3")]
    RedSpawn3,
    #[serde(rename = "readspawn4")]
    RedSpawn4,
    #[serde(rename = "bluespawn")]
    BluSpawn,
    #[serde(rename = "bluespawn1")]
    BluSpawn1,
    #[serde(rename = "bluespawn2")]
    BluSpawn2,
    #[serde(rename = "bluespawn3")]
    BluSpawn3,
    #[serde(rename = "bluespawn4")]
    BluSpawn4,
    #[serde(rename = "redintel")]
    RedIntel,
    #[serde(rename = "blueintel")]
    BluIntel,
    #[serde(rename = "redteamgate")]
    RedTeamGate,
    #[serde(rename = "blueteamgate")]
    BluTeamGate,
    #[serde(rename = "redteamgate2")]
    RedTeamGate2,
    #[serde(rename = "blueteamgate2")]
    BluTeamGate2,
    #[serde(rename = "redintelgate")]
    RedIntelGate,
    #[serde(rename = "blueintelgate")]
    BluIntelGate,
    #[serde(rename = "redintelgate2")]
    RedIntelGate2,
    #[serde(rename = "blueintelgate2")]
    BluIntelGate2,
    #[serde(rename = "intelgatehorizontal")]
    IntelGateHorizontal,
    #[serde(rename = "intelgatevertical")]
    IntelGateVertical,
    #[serde(rename = "medCabinet")]
    MedicalCabinet,
    #[serde(rename = "killbox")]
    KillBox,
    #[serde(rename = "pitfall")]
    PitFall,
    #[serde(rename = "fragbox")]
    FragBox,
    #[serde(rename = "playerwall")]
    PlayerWall,
    #[serde(rename = "playerwall_horizontal")]
    PlayerWallHorizontal,
    #[serde(rename = "bulletwall")]
    BulletWall,
    #[serde(rename = "bulletwall_horizontal")]
    BulletWallHorizontal,
    #[serde(rename = "leftdoor")]
    LeftDoor,
    #[serde(rename = "rightdoor")]
    RightDoor,
    #[serde(rename = "controlPoint1")]
    ControlPoint1,
    #[serde(rename = "controlPoint2")]
    ControlPoint2,
    #[serde(rename = "controlPoint3")]
    ControlPoint3,
    #[serde(rename = "controlPoint4")]
    ControlPoint4,
    #[serde(rename = "controlPoint5")]
    ControlPoint5,
    #[serde(rename = "NextArea0")]
    NextArea,
    CapturePoint,
    SetupGate,
    ArenaControlPoint,
    GeneratorRed,
    GeneratorBlue,
    MoveBoxUp,
    MoveBoxDown,
    MoveBoxLeft,
    MoveBoxRight,
    KothControlPoint,
    KothRedControlPoint,
    KothBlueControlPoint,
    #[serde(rename = "dropdownPlatform")]
    DropDownPlatform,
    #[serde(rename = "foreground")]
    Foreground,
    #[serde(rename = "foreground_scale")]
    ForegroundScale,
    #[serde(rename = "moving_platform")]
    MovingPlatform,
}
