use crate::map::{entity::MapEntity, io::error::MapIoError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamemode {
    Arena,
    /// Abbreviation: CTF
    CaptureTheFlag,
    /// Abbreviation: CP
    ControlPoint,
    /// Abbreviation: ADCP
    AttackDefenceControlPoint,
    /// Abbreviation: KOTH
    KingOfTheHill,
    /// Abbreviation: DKOTH
    DualKingOfTheHill,
    /// Abbreviation: Gen
    Generator,
    //Inavsion,
    /// Abbreviation: TDM
    TeamDeathmatch,
}

const CONTROL_POINTS_11111: [bool; 5] = [true, true, true, true, true];
const CONTROL_POINTS_10000: [bool; 5] = [true, false, false, false, false];

#[derive(Default)]
struct GamemodeMapScan {
    red_intel: usize,
    blu_intel: usize,
    control_points: [bool; 5],
    capture_point: bool,
    setup_gate: bool,
    koth_control_point: usize,
    koth_red_control_point: usize,
    koth_blu_control_point: usize,
    arena_control_poin: usize,
    generator_red: usize,
    generator_blu: usize,
}

impl Gamemode {
    /// Figures out which gamemode the map entities are
    pub fn scan_map_entities(entities: &[MapEntity]) -> Result<Self, MapIoError> {
        let mut scan = GamemodeMapScan::default();

        for entity in entities {
            match entity {
                MapEntity::RedIntel(_) => scan.red_intel += 1,
                MapEntity::BluIntel(_) => scan.blu_intel += 1,
                MapEntity::ControlPoint1(_) => scan.control_points[0] = true,
                MapEntity::ControlPoint2(_) => scan.control_points[1] = true,
                MapEntity::ControlPoint3(_) => scan.control_points[2] = true,
                MapEntity::ControlPoint4(_) => scan.control_points[3] = true,
                MapEntity::ControlPoint5(_) => scan.control_points[4] = true,
                MapEntity::CapturePoint(_) => scan.capture_point = true,
                MapEntity::SetupGate(_) => scan.setup_gate = true,
                MapEntity::KothControlPoint(_) => scan.koth_control_point += 1,
                MapEntity::KothRedControlPoint(_) => scan.koth_red_control_point += 1,
                MapEntity::KothBlueControlPoint(_) => scan.koth_blu_control_point += 1,
                MapEntity::ArenaControlPoint(_) => scan.arena_control_poin += 1,
                MapEntity::GeneratorRed(_) => scan.generator_red += 1,
                MapEntity::GeneratorBlue(_) => scan.generator_blu += 1,
                _ => (),
            }
        }

        match scan {
            GamemodeMapScan {
                red_intel: 1,
                blu_intel: 1,
                ..
            } => Ok(Self::CaptureTheFlag),
            GamemodeMapScan {
                control_points: CONTROL_POINTS_11111,
                capture_point: true,
                setup_gate: true,
                ..
            } => Ok(Self::AttackDefenceControlPoint),
            GamemodeMapScan {
                control_points: CONTROL_POINTS_11111,
                capture_point: true,
                ..
            } => Ok(Self::ControlPoint),
            GamemodeMapScan {
                control_points: CONTROL_POINTS_10000,
                capture_point: true,
                koth_control_point: 1,
                ..
            } => Ok(Self::KingOfTheHill),
            GamemodeMapScan {
                control_points: CONTROL_POINTS_10000,
                capture_point: true,
                koth_red_control_point: 1,
                koth_blu_control_point: 1,
                ..
            } => Ok(Self::DualKingOfTheHill),
            GamemodeMapScan {
                arena_control_poin: 1,
                capture_point: true,
                ..
            } => Ok(Self::Arena),
            GamemodeMapScan {
                generator_red: 1,
                generator_blu: 1,
                ..
            } => Ok(Self::Generator),
            _ => Err(MapIoError::Gamemode),
        }
    }
}
