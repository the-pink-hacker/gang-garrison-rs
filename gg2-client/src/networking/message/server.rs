use std::time::Duration;

use gg2_common::{
    intel::RawIntel,
    networking::{
        PacketKind,
        error::{Error, Result},
        message::*,
    },
    player::{PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo, team::Captures},
};

use super::ClientNetworkDeserialize;

macro_rules! generic_enum {
    (pub enum $name:ident {$($case:ident),+$(,)?}) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($case(concat_idents!(Server, $case))),+,
        }
    };
}

generic_enum!(
    pub enum ServerMessageGeneric {
        Hello,
        PlayerJoin,
        PlayerLeave,
        PlayerChangeTeam,
        PlayerChangeClass,
        PlayerSpawn,
        InputState,
        ChangeMap,
        FullUpdate,
        QuickUpdate,
        PlayerDeath,
        ServerFull,
        //RedTeamCap = 12,
        //BlueTeamCap = 13,
        //MapEnd = 14,
        //ChatBubble = 15,
        //BuildSentry = 16,
        //DestroySentry = 17,
        //Balance = 18,
        GrabIntel,
        ScoreIntel,
        DropIntel,
        //UberCharged = 22,
        //Uber = 23,
        //Omnomnomnom = 24,
        PasswordRequest,
        PasswordWrong,
        CaptureUpdate,
        //CpCaptured = 30,
        PlayerChangeName,
        //GeneratorDestroy = 32,
        //ArenaWaitForPlayers = 33,
        //ArenaEndround = 34,
        //ArenaRestart = 35,
        //UnlockCp = 36,
        //ServerKick = 37,
        //Kick = 38,
        //KickName = 39,
        //ArenaStartround = 40,
        //ToggleZoom = 41,
        ReturnIntel,
        IncompatibleProtocol,
        JoinUpdate,
        //DownloadMap = 45,
        //SentryPosition = 46,
        //RewardUpdate = 47,
        //RewardRequest = 50,
        //RewardChallengeCode = 51,
        //RewardChallengeResponse = 52,
        MessageString,
        WeaponFire,
        //PluginPacket = 55,
        //KickBadPluginPacket = 56,
        //Ping = 57,
        //ClientSettings = 58,
        //KickMultiClient = 59,
        ReserveSlot,
    }
);

/// Significantly reduce vebosity with `ServerMessageGeneric::take`
macro_rules! generic_match {
    ($buffer:ident, $kind:ident, [$($case:ident),+$(,)?]$(,)?) => {
        match $kind {
            $(PacketKind::$case => Ok(ServerMessageGeneric::$case(<concat_idents!(Server, $case)>::deserialize($buffer)?))),+,
            _ => Err(Error::IncorrectMessage($kind)),
        }
    };
}

impl ServerMessageGeneric {
    pub fn take<I: Iterator<Item = u8>>(buffer: &mut I) -> Result<Self> {
        let raw_kind = buffer.read_u8()?;
        let kind = raw_kind
            .try_into()
            .map_err(|_| Error::PacketKind(raw_kind))?;

        generic_match!(
            buffer,
            kind,
            [
                Hello,
                PlayerJoin,
                PlayerLeave,
                PlayerChangeTeam,
                PlayerChangeClass,
                PlayerSpawn,
                InputState,
                ChangeMap,
                FullUpdate,
                QuickUpdate,
                PlayerDeath,
                ServerFull,
                GrabIntel,
                ScoreIntel,
                DropIntel,
                PasswordRequest,
                PasswordWrong,
                CaptureUpdate,
                PlayerChangeName,
                ReturnIntel,
                IncompatibleProtocol,
                JoinUpdate,
                MessageString,
                WeaponFire,
                ReserveSlot,
            ],
        )
    }
}

macro_rules! generic_kind_match {
    ($value:ident, [$($case:ident),+$(,)?]$(,)?) => {
        match $value {
            $(ServerMessageGeneric::$case(_) => PacketKind::$case),+,
        }
    };
}

impl From<ServerMessageGeneric> for PacketKind {
    fn from(value: ServerMessageGeneric) -> Self {
        generic_kind_match!(
            value,
            [
                Hello,
                PlayerJoin,
                PlayerLeave,
                PlayerChangeTeam,
                PlayerChangeClass,
                PlayerSpawn,
                InputState,
                ChangeMap,
                FullUpdate,
                QuickUpdate,
                PlayerDeath,
                ServerFull,
                GrabIntel,
                ScoreIntel,
                DropIntel,
                PasswordRequest,
                PasswordWrong,
                CaptureUpdate,
                PlayerChangeName,
                ReturnIntel,
                IncompatibleProtocol,
                JoinUpdate,
                MessageString,
                WeaponFire,
                ReserveSlot,
            ],
        )
    }
}

impl ClientNetworkDeserialize for Captures {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let red_captures = payload.read_u8()?;
        let blu_captures = payload.read_u8()?;

        let raw_respawn_time = payload.read_u8()?;
        let respawn_time = Duration::from_secs(raw_respawn_time as u64);

        Ok(Self {
            red_captures,
            blu_captures,
            respawn_time,
        })
    }
}

impl ClientNetworkDeserialize for ServerCaptureUpdate {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_amount = payload.read_u8()?;
        let captures = Captures::deserialize(payload)?;

        // TODO: HUD
        payload.next();
        payload.next();
        payload.next();
        payload.next();
        payload.next();

        Ok(Self {
            player_amount,
            captures,
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

impl ClientNetworkDeserialize for ServerDropIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerGrabIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
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
        let position = payload.read_fixed_point_u16_vec2(5.0)?;
        let velocity = payload.read_fixed_point_u8_vec2(8.5)?;
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
        let captures = payload.read_u8()?;
        let assists = payload.read_u8()?;
        let destruction = payload.read_u8()?;
        let stabs = payload.read_u8()?;
        let healing = payload.read_u16()?;
        let defenses = payload.read_u8()?;
        let invulnerability = payload.read_bool()?;
        let bonus = payload.read_u8()?;
        let points = payload.read_u8()?;
        let queue_jump = payload.read_bool()?;
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
            captures,
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

        // TODO: Moving platform
        //payload.next();
        //payload.next();
        //payload.next();
        //payload.next();
        //payload.next();

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

        let capture_limit = payload.read_u8()?;
        let captures = Captures::deserialize(payload)?;

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
            capture_limit,
            captures,
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

        let _plugins_amounts = payload.next().ok_or(Error::UnexpectedEOF)?;
        let _plugins_raw = payload.read_utf8_long_string()?;

        Ok(Self {
            server_name,
            map_name,
            map_md5,
            plugins: Vec::new(),
        })
    }
}

impl ClientNetworkDeserialize for ServerIncompatibleProtocol {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
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

impl ClientNetworkDeserialize for ServerPasswordRequest {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerPasswordWrong {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeClass {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;
        let player_class = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self {
            player_id,
            player_class,
        })
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeName {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;
        let name = payload.read_utf8_short_string()?;

        Ok(Self { player_id, name })
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeTeam {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;
        let player_team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self {
            player_id,
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
        let player_id = payload.read_u8()?.try_into()?;
        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerPlayerSpawn {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;
        let spawn_index = payload.read_u8()?;
        let spawn_group = payload.read_u8()?;

        Ok(Self {
            player_id,
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

        // TODO: Moving platform
        //payload.next();
        //payload.next();
        //payload.next();
        //payload.next();
        //payload.next();

        Ok(Self { player_characters })
    }
}

impl ClientNetworkDeserialize for ServerReserveSlot {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerReturnIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self { team })
    }
}

impl ClientNetworkDeserialize for ServerScoreIntel {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerServerFull {
    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerWeaponFire {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_id = payload.read_u8()?.try_into()?;
        let position = payload.read_fixed_point_u16_vec2(5.0)?;
        let velocity = payload.read_fixed_point_u8_vec2(8.5)?;
        let seed = payload.read_u16()?;

        Ok(Self {
            player_id,
            position,
            velocity,
            seed,
        })
    }
}
