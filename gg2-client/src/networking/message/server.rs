use std::time::Duration;

use gg2_common::{
    error::{CommonError, Result},
    game::{control_point::RawControlPoint, generator::RawGenerator, intel::RawIntel},
    gamemode::Gamemode,
    hud::{GamemodeHud, GamemodeHudArenaFull, HudKothTimer, HudMatchTimer},
    networking::{PacketKind, error::NetworkError as Error, message::*},
    player::{PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo, team::Captures},
};

use super::{ClientNetworkDeserializationContext, ClientNetworkDeserialize};

macro_rules! generic_message {
    ($name:ident {$($case:ident),+$(,)?}) => {
        impl ClientNetworkDeserialize for ServerMessageGeneric {
            async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self> where I: Iterator<Item = u8>, C: ClientNetworkDeserializationContext {
                let raw_kind = payload.read_u8()?;
                let kind = raw_kind
                    .try_into()
                    .map_err(|_| Error::PacketKind(raw_kind))?;

                match kind {
                    $(PacketKind::$case => Ok(ServerMessageGeneric::$case(${concat(Server, $case)}::deserialize(payload, context).await?))),+,
                    _ => Err(CommonError::Network(Error::IncorrectMessage(kind))),
                }
            }
        }
    };
}

generic_message!(ServerMessageGeneric {
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
    ChatBubble,
    //BuildSentry = 16,
    //DestroySentry = 17,
    //Balance = 18,
    GrabIntel,
    ScoreIntel,
    DropIntel,
    //UberCharged = 22,
    //Uber = 23,
    Omnom,
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
});

impl ClientNetworkDeserialize for Captures {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let red_captures = payload.read_u8()?;
        let blu_captures = payload.read_u8()?;
        let respawn_time = payload.read_u8().map(u64::from).map(Duration::from_secs)?;

        Ok(Self {
            red_captures,
            blu_captures,
            respawn_time,
        })
    }
}

impl ClientNetworkDeserialize for HudMatchTimer {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let start = payload.read_u8().map(u64::from).map(Duration::from_mins)?;
        let current = payload
            .read_u32()
            .map(|time| time as u64 / (30 * 60))
            .map(Duration::from_secs)?;

        Ok(Self { start, current })
    }
}

impl ClientNetworkDeserialize for RawGenerator {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let health = payload.read_u16()?;
        let shield_health = payload.read_u16()?;

        Ok(Self {
            health,
            shield_health,
        })
    }
}

impl ClientNetworkDeserialize for GamemodeHudArenaFull {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let red_wins = payload.read_u8()?;
        let blu_wins = payload.read_u8()?;
        let state = payload.read_u8()?;
        let winners = payload.read_u8()?;
        let end_count = payload.read_u16()?;

        Ok(Self {
            red_wins,
            blu_wins,
            state,
            winners,
            end_count,
        })
    }
}

impl ClientNetworkDeserialize for RawControlPoint {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;
        let capturing_team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;
        let capturing = payload.read_u16()?;

        Ok(Self {
            team,
            capturing_team,
            capturing,
        })
    }
}

impl ClientNetworkDeserialize for HudKothTimer {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let capture_unlock = payload.read_duration_u16_sec()?;
        let red_timer = payload.read_duration_u16_sec()?;
        let blu_timer = payload.read_duration_u16_sec()?;

        Ok(Self {
            capture_unlock,
            red_timer,
            blu_timer,
        })
    }
}

trait GamemodeHudDeserialization: Sized {
    async fn deserialize<I, C>(payload: &mut I, context: &C, full_update: bool) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext;
}

impl GamemodeHudDeserialization for GamemodeHud {
    async fn deserialize<I, C>(payload: &mut I, context: &C, full_update: bool) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let current_gamemode = context.current_map_gamemode().await?;

        match current_gamemode {
            Gamemode::Arena => {
                let full_update = if full_update {
                    Some(GamemodeHudArenaFull::deserialize(payload, context).await?)
                } else {
                    None
                };

                let match_timer = HudMatchTimer::deserialize(payload, context).await?;
                let control_point_unlock = payload.read_duration_u16_sec()?;
                let round_start = payload.read_u8()?;
                let control_point = RawControlPoint::deserialize(payload, context).await?;

                Ok(Self::Arena {
                    full_update,
                    match_timer,
                    control_point_unlock,
                    round_start,
                    control_point,
                })
            }
            Gamemode::CaptureTheFlag => {
                let match_timer = HudMatchTimer::deserialize(payload, context).await?;

                Ok(Self::CaptureTheFlag { match_timer })
            }
            Gamemode::ControlPoint | Gamemode::AttackDefenceControlPoint => {
                let match_timer = HudMatchTimer::deserialize(payload, context).await?;
                let setup_timer = payload.read_duration_u16_sec()?;

                let control_points_length = context.current_map_control_points_length().await?;
                let mut control_points = Vec::with_capacity(control_points_length as usize);

                for _ in 0..control_points_length {
                    control_points.push(RawControlPoint::deserialize(payload, context).await?);
                }

                Ok(Self::ControlPoint {
                    match_timer,
                    setup_timer,
                    control_points,
                })
            }
            Gamemode::KingOfTheHill => {
                let timer = HudKothTimer::deserialize(payload, context).await?;
                let control_point = RawControlPoint::deserialize(payload, context).await?;

                Ok(Self::KingOfTheHill {
                    timer,
                    control_point,
                })
            }
            Gamemode::DualKingOfTheHill => {
                let timer = HudKothTimer::deserialize(payload, context).await?;
                let red_control_point = RawControlPoint::deserialize(payload, context).await?;
                let blu_control_point = RawControlPoint::deserialize(payload, context).await?;

                Ok(Self::DualKingOfTheHill {
                    timer,
                    red_control_point,
                    blu_control_point,
                })
            }
            Gamemode::Generator => {
                let match_timer = HudMatchTimer::deserialize(payload, context).await?;
                let blu_generator = RawGenerator::deserialize(payload, context).await?;
                let red_generator = RawGenerator::deserialize(payload, context).await?;

                Ok(Self::Generator {
                    match_timer,
                    blu_generator,
                    red_generator,
                })
            }
            Gamemode::Inavsion => {
                let match_timer = HudMatchTimer::deserialize(payload, context).await?;
                let setup_timer = payload.read_duration_u16_sec()?;

                Ok(Self::Invasion {
                    match_timer,
                    setup_timer,
                })
            }
            Gamemode::TeamDeathmatch => {
                let match_timer = HudMatchTimer::deserialize(payload, context).await?;
                let kill_limit = payload.read_u16()?;

                Ok(Self::TeamDeathmatch {
                    match_timer,
                    kill_limit,
                })
            }
        }
    }
}

impl ClientNetworkDeserialize for ServerCaptureUpdate {
    async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_amount = payload.read_u8()?;
        let captures = Captures::deserialize(payload, context).await?;
        let hud = GamemodeHud::deserialize(payload, context, false).await?;

        Ok(Self {
            player_amount,
            captures,
            hud,
        })
    }
}

impl ClientNetworkDeserialize for ServerChangeMap {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let map_name = payload.read_utf8_short_string()?;
        let map_md5 = payload.read_md5()?;

        if map_name.chars().by_ref().all(char::is_alphanumeric) {
            Err(CommonError::Network(Error::UnsanitizedString))
        } else {
            Ok(Self { map_name, map_md5 })
        }
    }
}

impl ClientNetworkDeserialize for ServerChatBubble {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        // TODO: What does this byte do?
        let _unknown = payload.read_u8()?;

        let bubble = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self { bubble })
    }
}

impl ClientNetworkDeserialize for ServerDropIntel {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerGrabIntel {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for RawInput {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let translation = payload.read_fixed_point_u16_vec2(5.0)?;
        let velocity = payload.read_fixed_point_u8_vec2(8.5)?;
        let health = payload.read_u8()?;
        let ammo_count = payload.read_u8()?;
        let move_status = payload.read_u8()?;

        Ok(Self {
            translation,
            velocity,
            health,
            ammo_count,
            move_status,
        })
    }
}

impl ClientNetworkDeserialize for RawAdditionalPlayerInfo {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        // TODO: Implement additional player info
        for _ in 0..9 {
            payload.next();
        }

        Ok(Self {})
    }
}

/// TODO: Merge player_length into context
trait DeserializePlayerUpdateInfo: Sized {
    async fn deserialize<I, C>(payload: &mut I, context: &C, player_length: u8) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext;
}

impl DeserializePlayerUpdateInfo for PlayerUpdateInfo {
    async fn deserialize<I, C>(payload: &mut I, context: &C, player_length: u8) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
            let input = RawInput::deserialize(payload, context).await?;
            let player_info = RawPlayerInfo::deserialize(payload, context).await?;
            let additional_into = RawAdditionalPlayerInfo::deserialize(payload, context).await?;
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let position = payload.read_fixed_point_u16_vec2(5.0)?;
        let _recharge_time = payload.read_u16()? as i16;
        Ok(Self {
            position,
            recharge_time: Duration::default(),
        })
    }
}

impl ClientNetworkDeserialize for ServerFullUpdate {
    async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let team_death_match_invulnerability_ticks = payload.read_u16()?;
        let player_length = payload.read_u8()?;

        let mut player_info = Vec::with_capacity(player_length as usize);

        for _ in 0..player_length {
            player_info.push(PlayerUpdateInfo::deserialize(payload, context, player_length).await?);
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
            red_intel.push(RawIntel::deserialize(payload, context).await?);
        }

        let blu_intel_length = payload.read_u16()?;
        let mut blu_intel = Vec::with_capacity(blu_intel_length as usize);

        for _ in 0..blu_intel_length {
            blu_intel.push(RawIntel::deserialize(payload, context).await?);
        }

        let capture_limit = payload.read_u8()?;
        let captures = Captures::deserialize(payload, context).await?;
        let hud = GamemodeHud::deserialize(payload, context, true).await?;

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
            hud,
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(_payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerInputState {
    async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let character_length = payload.read_u8()?;
        let mut inputs = Vec::with_capacity(character_length as usize);

        for _ in 0..character_length {
            let has_character = payload.read_bool()?;
            let input = if has_character {
                Some(RawInput::deserialize(payload, context).await?)
            } else {
                None
            };

            inputs.push(input);
        }

        Ok(Self { inputs })
    }
}

impl ClientNetworkDeserialize for ServerJoinUpdate {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let client_player_id = payload.read_u8()?.try_into()?;
        let map_area = payload.read_u8()?;
        Ok(ServerJoinUpdate {
            client_player_id,
            map_area,
        })
    }
}

impl ClientNetworkDeserialize for ServerMessageString {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let message = payload.read_utf8_short_string()?;
        Ok(Self { message })
    }
}

impl ClientNetworkDeserialize for ServerOmnom {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let _unknown = payload.read_u8()?;
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerPasswordRequest {
    async fn deserialize<I, C>(_payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerPasswordWrong {
    async fn deserialize<I, C>(_payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeClass {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_id = payload.read_u8()?.try_into()?;
        let name = payload.read_utf8_short_string()?;

        Ok(Self { player_id, name })
    }
}

impl ClientNetworkDeserialize for ServerPlayerChangeTeam {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_name = payload.read_utf8_short_string()?;

        Ok(ServerPlayerJoin { player_name })
    }
}

impl ClientNetworkDeserialize for ServerPlayerLeave {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_id = payload.read_u8()?.try_into()?;
        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerPlayerSpawn {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
    async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_length = payload.read_u8()?;

        let mut player_characters = Vec::with_capacity(player_length.into());

        for _ in 0..player_length {
            let character_present = payload.read_bool()?;
            let character = if character_present {
                let input = RawInput::deserialize(payload, context).await?;
                let player_info = RawPlayerInfo::deserialize(payload, context).await?;

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
    async fn deserialize<I, C>(_payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerReturnIntel {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let team = payload
            .read_u8()?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;

        Ok(Self { team })
    }
}

impl ClientNetworkDeserialize for ServerScoreIntel {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        let player_id = payload.read_u8()?.try_into()?;

        Ok(Self { player_id })
    }
}

impl ClientNetworkDeserialize for ServerServerFull {
    async fn deserialize<I, C>(_payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
        Ok(Self)
    }
}

impl ClientNetworkDeserialize for ServerWeaponFire {
    async fn deserialize<I, C>(payload: &mut I, _context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext,
    {
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
