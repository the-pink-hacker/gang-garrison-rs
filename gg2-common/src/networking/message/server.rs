use std::time::Duration;

use bevy::prelude::*;

use crate::{
    intel::RawIntel,
    networking::{
        error::{Error, Result},
        PacketKind,
    },
    player::{Class, Team},
};

use super::{GGMessage, MessageReader};

#[derive(Debug, Clone)]
pub struct ServerHello {
    pub server_name: String,
    pub map_name: String,
    pub map_md5: Option<u128>,
    pub plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

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

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(ServerReserveSlot)
    }
}

#[derive(Debug, Clone)]
pub struct ServerServerFull;

impl GGMessage for ServerServerFull {
    const KIND: PacketKind = PacketKind::ServerFull;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(ServerServerFull)
    }
}

// TODO: Implement inputstate
#[derive(Debug, Clone)]
pub struct ServerInputState;

impl GGMessage for ServerInputState {
    const KIND: PacketKind = PacketKind::InputState;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        Ok(ServerInputState {})
    }
}

// TODO: Implement quick update
#[derive(Debug, Clone)]
pub struct ServerQuickUpdate;

impl GGMessage for ServerQuickUpdate {
    const KIND: PacketKind = PacketKind::QuickUpdate;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        Ok(ServerQuickUpdate {})
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerJoin {
    pub player_name: String,
}

impl GGMessage for ServerPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_name = payload.read_utf8_short_string()?;

        Ok(ServerPlayerJoin { player_name })
    }
}

#[derive(Debug, Clone)]
pub struct ServerJoinUpdate {
    pub amount_of_players: u8,
    pub map_area: u8,
}

impl GGMessage for ServerJoinUpdate {
    const KIND: PacketKind = PacketKind::JoinUpdate;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let amount_of_players = payload.read_u8()?;
        let map_area = payload.read_u8()?;
        Ok(ServerJoinUpdate {
            amount_of_players,
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

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let map_name = payload.read_utf8_short_string()?;
        let map_md5 = payload.read_md5()?;
        Ok(Self { map_name, map_md5 })
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerChangeClass {
    pub player_index: u8,
    pub player_class: Class,
}

impl GGMessage for ServerPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?;
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
    pub player_index: u8,
    pub player_team: Team,
}

impl GGMessage for ServerPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?;
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
    pub subobjects: Vec<()>,
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

        let non_current_players = if player_length == 0 {
            0
        } else {
            player_length - 1
        };

        let dominations = payload
            .take(non_current_players as usize)
            .collect::<Vec<_>>();

        let subobjects_length = payload.read_u8()?;
        // TODO: Add subobjects
        assert_eq!(subobjects_length, 0);

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
            subobjects: Vec::new(),
        })
    }
}

impl RawIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let x = payload.read_fixed_point_u16()?;
        let y = payload.read_fixed_point_u16()?;
        let _recharge_time = payload.read_u16()? as i16;
        Ok(Self {
            position: Vec2::new(x, y),
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

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let team_death_match_invulnerability_ticks = payload.read_u16()?;
        let player_length = payload.read_u8()?;

        let mut player_info = Vec::with_capacity(player_length as usize);

        for _ in 0..player_length {
            player_info.push(PlayerUpdateInfo::deserialize(payload, player_length)?);
        }

        let red_intel_length = payload.read_u8()?;
        let mut red_intel = Vec::with_capacity(red_intel_length as usize);

        for _ in 0..red_intel_length {
            red_intel.push(RawIntel::deserialize(payload)?);
        }

        let blu_intel_length = payload.read_u8()?;
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

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let message = payload.read_utf8_short_string()?;
        Ok(Self { message })
    }
}
