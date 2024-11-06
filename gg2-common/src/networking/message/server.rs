use std::time::Duration;

use bevy::prelude::*;

use crate::{
    intel::RawIntel,
    networking::{
        error::{Error, Result},
        PacketKind,
    },
    player::{
        class::ClassGeneric, team::Team, PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo,
    },
};

use super::{GGMessage, MessageReader, NetworkDeserialize};

#[derive(Debug, Clone)]
pub struct ServerHello {
    pub server_name: String,
    pub map_name: String,
    pub map_md5: Option<u128>,
    pub plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;
}

impl NetworkDeserialize for ServerHello {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let server_name = payload.read_utf8_short_string()?;
        let map_name = payload.read_utf8_short_string()?;

        let map_md5 = payload.read_md5()?;

        let plugins_amounts = payload.next().ok_or(Error::UnexpectedEOF)?;
        let plugins_raw = payload.read_utf8_long_string()?;
        println!("Found {} plugins: [ {} ]", plugins_amounts, plugins_raw);

        Ok(Self {
            server_name,
            map_name,
            map_md5,
            plugins: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerReserveSlot;

impl GGMessage for ServerReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}

impl NetworkDeserialize for ServerReserveSlot {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug, Clone)]
pub struct ServerServerFull;

impl GGMessage for ServerServerFull {
    const KIND: PacketKind = PacketKind::ServerFull;
}

impl NetworkDeserialize for ServerServerFull {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug, Clone)]
pub struct ServerInputState {
    pub inputs: Vec<Option<RawInput>>,
}

impl GGMessage for ServerInputState {
    const KIND: PacketKind = PacketKind::InputState;
}

impl NetworkDeserialize for ServerInputState {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let character_length = payload.read_u8()?;
        let mut inputs = Vec::with_capacity(character_length as usize);

        for _ in 0..character_length {
            let has_character = payload.read_bool()?;
            let input = if has_character {
                Some(RawInput::deserialize(payload)?)
            } else {
                None
            };

            inputs.push(input);
        }

        Ok(Self { inputs })
    }
}

#[derive(Debug, Clone)]
pub struct ServerQuickUpdate {
    pub player_characters: Vec<Option<(RawInput, RawPlayerInfo)>>,
}

impl GGMessage for ServerQuickUpdate {
    const KIND: PacketKind = PacketKind::QuickUpdate;
}

impl NetworkDeserialize for ServerQuickUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_length = payload.read_u8()?;

        let mut player_characters = Vec::with_capacity(player_length.into());

        for _ in 0..player_length {
            let character_present = payload.read_bool()?;
            let character = if character_present {
                let input = RawInput::deserialize(payload)?;
                let player_info = RawPlayerInfo::deserialize(payload)?;

                Some((input, player_info))
            } else {
                None
            };

            player_characters.push(character);
        }

        Ok(Self { player_characters })
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerJoin {
    pub player_name: String,
}

impl GGMessage for ServerPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

impl NetworkDeserialize for ServerPlayerJoin {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_name = payload.read_utf8_short_string()?;

        Ok(ServerPlayerJoin { player_name })
    }
}

#[derive(Debug, Clone)]
pub struct ServerJoinUpdate {
    pub client_player_id: PlayerId,
    pub map_area: u8,
}

impl GGMessage for ServerJoinUpdate {
    const KIND: PacketKind = PacketKind::JoinUpdate;
}

impl NetworkDeserialize for ServerJoinUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let client_player_id = payload.read_u8()?.into();
        let map_area = payload.read_u8()?;
        Ok(ServerJoinUpdate {
            client_player_id,
            map_area,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerChangeMap {
    pub map_name: String,
    pub map_md5: Option<u128>,
}

impl GGMessage for ServerChangeMap {
    const KIND: PacketKind = PacketKind::ChangeMap;
}

impl NetworkDeserialize for ServerChangeMap {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let map_name = payload.read_utf8_short_string()?;
        let map_md5 = payload.read_md5()?;

        if map_name.chars().by_ref().all(char::is_alphanumeric) {
            Err(Error::UnsanitizedString)
        } else {
            Ok(Self { map_name, map_md5 })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerChangeClass {
    pub player_index: PlayerId,
    pub player_class: ClassGeneric,
}

impl GGMessage for ServerPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

impl NetworkDeserialize for ServerPlayerChangeClass {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.into();
        let player_class = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self {
            player_index,
            player_class,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerChangeTeam {
    pub player_index: PlayerId,
    pub player_team: Team,
}

impl GGMessage for ServerPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

impl NetworkDeserialize for ServerPlayerChangeTeam {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.into();
        let player_team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self {
            player_index,
            player_team,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PlayerUpdateInfo {
    pub kills: u8,
    pub deaths: u8,
    pub caps: u8,
    pub assists: u8,
    pub destruction: u8,
    pub stabs: u8,
    pub healing: u16,
    pub defenses: u8,
    pub invulnerability: u8,
    pub bonus: u8,
    pub points: u8,
    pub queue_jump: u8,
    pub rewards: String,
    pub dominations: Vec<u8>,
    pub character: Option<(RawInput, RawPlayerInfo, RawAdditionalPlayerInfo)>,
}

impl RawInput {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let key_state = payload.read_u8()?;
        let net_aim_direction = payload.read_u16()?;
        let aim_distance = payload.read_fixed_point_u8(0.5)?;

        Ok(Self {
            key_state,
            net_aim_direction,
            aim_distance,
        })
    }
}

impl RawPlayerInfo {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let position = payload.read_fixed_point_u16_vec2(5.0)? * Vec2::new(1.0, -1.0);

        let velocity_x = payload.read_u8()? as i8 as f32 / 8.5;
        let velocity_y = payload.read_u8()? as i8 as f32 / -8.5;
        let velocity = Vec2::new(velocity_x, velocity_y);

        let health = payload.read_u8()?;
        let ammo_count = payload.read_u8()?;
        let move_status = payload.read_u8()?;

        Ok(Self {
            position,
            velocity,
            health,
            ammo_count,
            move_status,
        })
    }
}

impl RawAdditionalPlayerInfo {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        // TODO: Implement additional player info
        for _ in 0..9 {
            payload.next();
        }

        Ok(Self {})
    }
}

impl PlayerUpdateInfo {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I, player_length: u8) -> Result<Self> {
        let kills = payload.read_u8()?;
        let deaths = payload.read_u8()?;
        let caps = payload.read_u8()?;
        let assists = payload.read_u8()?;
        let destruction = payload.read_u8()?;
        let stabs = payload.read_u8()?;
        let healing = payload.read_u16()?;
        let defenses = payload.read_u8()?;
        let invulnerability = payload.read_u8()?;
        let bonus = payload.read_u8()?;
        let points = payload.read_u8()?;
        let queue_jump = payload.read_u8()?;
        let rewards = payload.read_utf8_long_string()?;

        let non_current_players = player_length.saturating_sub(1);
        let dominations = payload.take(non_current_players as usize).collect();

        let character_present = payload.read_bool()?;
        let character = if character_present {
            let input = RawInput::deserialize(payload)?;
            let player_info = RawPlayerInfo::deserialize(payload)?;
            let additional_into = RawAdditionalPlayerInfo::deserialize(payload)?;
            Some((input, player_info, additional_into))
        } else {
            None
        };

        Ok(Self {
            kills,
            deaths,
            caps,
            assists,
            destruction,
            stabs,
            healing,
            defenses,
            invulnerability,
            bonus,
            points,
            queue_jump,
            rewards,
            dominations,
            character,
        })
    }
}

impl RawIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let position = payload.read_fixed_point_u16_vec2(5.0)?;
        let _recharge_time = payload.read_u16()? as i16;
        Ok(Self {
            position,
            recharge_time: Duration::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerFullUpdate {
    pub team_death_match_invulnerability_ticks: u16,
    pub player_info: Vec<PlayerUpdateInfo>,
    pub red_intel: Vec<RawIntel>,
    pub blu_intel: Vec<RawIntel>,
    pub cap_limit: u8,
    pub red_cap: u8,
    pub blu_cap: u8,
    pub respawn_time: Duration,
    pub scout_limit: u8,
    pub soldier_limit: u8,
    pub sniper_limit: u8,
    pub demoman_limit: u8,
    pub medic_limit: u8,
    pub engineer_limit: u8,
    pub heavy_limit: u8,
    pub spy_limit: u8,
    pub pyro_limit: u8,
    pub quote_limit: u8,
}

impl GGMessage for ServerFullUpdate {
    const KIND: PacketKind = PacketKind::FullUpdate;
}

impl NetworkDeserialize for ServerFullUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let team_death_match_invulnerability_ticks = payload.read_u16()?;
        let player_length = payload.read_u8()?;

        let mut player_info = Vec::with_capacity(player_length as usize);

        for _ in 0..player_length {
            player_info.push(PlayerUpdateInfo::deserialize(payload, player_length)?);
        }

        let red_intel_length = payload.read_u16()?;
        let mut red_intel = Vec::with_capacity(red_intel_length as usize);

        for _ in 0..red_intel_length {
            red_intel.push(RawIntel::deserialize(payload)?);
        }

        let blu_intel_length = payload.read_u16()?;
        let mut blu_intel = Vec::with_capacity(blu_intel_length as usize);

        for _ in 0..blu_intel_length {
            blu_intel.push(RawIntel::deserialize(payload)?);
        }

        let cap_limit = payload.read_u8()?;
        let red_cap = payload.read_u8()?;
        let blu_cap = payload.read_u8()?;

        let raw_respawn_time = payload.read_u8()?;
        let respawn_time = Duration::from_secs(raw_respawn_time as u64);

        // TODO: HUD
        payload.next();
        payload.next();
        payload.next();
        payload.next();
        payload.next();

        let scout_limit = payload.read_u8()?;
        let soldier_limit = payload.read_u8()?;
        let sniper_limit = payload.read_u8()?;
        let demoman_limit = payload.read_u8()?;
        let medic_limit = payload.read_u8()?;
        let engineer_limit = payload.read_u8()?;
        let heavy_limit = payload.read_u8()?;
        let spy_limit = payload.read_u8()?;
        let pyro_limit = payload.read_u8()?;
        let quote_limit = payload.read_u8()?;

        Ok(Self {
            team_death_match_invulnerability_ticks,
            player_info,
            red_intel,
            blu_intel,
            cap_limit,
            red_cap,
            blu_cap,
            respawn_time,
            scout_limit,
            soldier_limit,
            sniper_limit,
            demoman_limit,
            medic_limit,
            engineer_limit,
            heavy_limit,
            spy_limit,
            pyro_limit,
            quote_limit,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ServerMessageString {
    pub message: String,
}

impl GGMessage for ServerMessageString {
    const KIND: PacketKind = PacketKind::MessageString;
}

impl NetworkDeserialize for ServerMessageString {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let message = payload.read_utf8_short_string()?;
        Ok(Self { message })
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerLeave {
    pub player_index: PlayerId,
}

impl GGMessage for ServerPlayerLeave {
    const KIND: PacketKind = PacketKind::PlayerLeave;
}

impl NetworkDeserialize for ServerPlayerLeave {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.into();
        Ok(Self { player_index })
    }
}
