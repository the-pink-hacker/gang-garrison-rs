use std::time::Duration;

use bevy::prelude::*;
use gg2_common::{
    intel::RawIntel,
    networking::{
        error::{Error, Result},
        message::*,
    },
    player::{team::Caps, PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo},
};

use super::ClientNetworkDeserialize;

impl ClientNetworkDeserialize for Caps {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let red_cap = payload.read_u8()?;
        let blu_cap = payload.read_u8()?;

        let raw_respawn_time = payload.read_u8()?;
        let respawn_time = Duration::from_secs(raw_respawn_time as u64);

        Ok(Self {
            red_cap,
            blu_cap,
            respawn_time,
        })
    }
}

impl ClientNetworkDeserialize for ServerCapsUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_amount = payload.read_u8()?;
        let caps = Caps::deserialize(payload)?;

        // TODO: HUD
        payload.next();
        payload.next();
        payload.next();
        payload.next();
        payload.next();

        Ok(Self {
            player_amount,
            caps,
        })
    }
}

impl ClientNetworkDeserialize for ServerChangeMap {
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

impl ClientNetworkDeserialize for RawInput {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let key_state = payload.read_u8()?.into();
        let net_aim_direction = payload.read_u16()?;
        let aim_distance = payload.read_fixed_point_u8(0.5)?;

        Ok(Self {
            key_state,
            net_aim_direction,
            aim_distance,
        })
    }
}

impl ClientNetworkDeserialize for RawPlayerInfo {
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

impl ClientNetworkDeserialize for RawAdditionalPlayerInfo {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        // TODO: Implement additional player info
        for _ in 0..9 {
            payload.next();
        }

        Ok(Self {})
    }
}

trait DeserializePlayerUpdateInfo {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I, player_length: u8) -> Result<Self>
    where
        Self: Sized;
}

impl DeserializePlayerUpdateInfo for PlayerUpdateInfo {
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

impl ClientNetworkDeserialize for RawIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let position = payload.read_fixed_point_u16_vec2(5.0)?;
        let _recharge_time = payload.read_u16()? as i16;
        Ok(Self {
            position,
            recharge_time: Duration::default(),
        })
    }
}

impl ClientNetworkDeserialize for ServerFullUpdate {
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
        let caps = Caps::deserialize(payload)?;

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
            caps,
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

impl ClientNetworkDeserialize for ServerHello {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let server_name = payload.read_utf8_short_string()?;
        let map_name = payload.read_utf8_short_string()?;

        let map_md5 = payload.read_md5()?;

        let plugins_amounts = payload.next().ok_or(Error::UnexpectedEOF)?;
        let plugins_raw = payload.read_utf8_long_string()?;
        debug!("Found {} plugins: [ {} ]", plugins_amounts, plugins_raw);

        Ok(Self {
            server_name,
            map_name,
            map_md5,
            plugins: Vec::new(),
        })
    }
}

impl ClientNetworkDeserialize for ServerInputState {
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

impl ClientNetworkDeserialize for ServerJoinUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let client_player_id = payload.read_u8()?.try_into()?;
        let map_area = payload.read_u8()?;
        Ok(ServerJoinUpdate {
            client_player_id,
            map_area,
        })
    }
}

impl ClientNetworkDeserialize for ServerMessageString {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let message = payload.read_utf8_short_string()?;
        Ok(Self { message })
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeClass {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.try_into()?;
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

impl ClientNetworkDeserialize for ServerPlayerChangeTeam {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.try_into()?;
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

impl ClientNetworkDeserialize for ServerPlayerDeath {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let target = payload.read_u8()?.try_into()?;
        let attacker = PlayerId::from_u8(payload.read_u8()?);
        let assist = PlayerId::from_u8(payload.read_u8()?);
        let damage_source = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self {
            target,
            attacker,
            assist,
            damage_source,
        })
    }
}

impl ClientNetworkDeserialize for ServerPlayerJoin {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_name = payload.read_utf8_short_string()?;

        Ok(ServerPlayerJoin { player_name })
    }
}

impl ClientNetworkDeserialize for ServerPlayerLeave {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.try_into()?;
        Ok(Self { player_index })
    }
}

impl ClientNetworkDeserialize for ServerPlayerSpawn {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.read_u8()?.try_into()?;
        let spawn_index = payload.read_u8()?;
        let spawn_group = payload.read_u8()?;

        Ok(Self {
            player_index,
            spawn_index,
            spawn_group,
        })
    }
}

impl ClientNetworkDeserialize for ServerQuickUpdate {
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

impl ClientNetworkDeserialize for ServerReserveSlot {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerServerFull {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}
