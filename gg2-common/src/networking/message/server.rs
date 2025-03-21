use crate::{
    damage::source::DamageSource,
    intel::RawIntel,
    networking::PacketKind,
    player::{
        class::ClassGeneric,
        team::{Caps, Team},
        PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo,
    },
};

use super::GGMessage;

#[derive(Debug, Clone)]
pub struct ServerCapsUpdate {
    pub player_amount: u8,
    pub caps: Caps,
}

impl GGMessage for ServerCapsUpdate {
    const KIND: PacketKind = PacketKind::CapsUpdate;
}

#[derive(Debug, Clone)]
pub struct ServerChangeMap {
    pub map_name: String,
    pub map_md5: Option<u128>,
}

impl GGMessage for ServerChangeMap {
    const KIND: PacketKind = PacketKind::ChangeMap;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerDeath {
    pub target: PlayerId,
    pub attacker: Option<PlayerId>,
    pub assist: Option<PlayerId>,
    pub damage_source: DamageSource,
}

impl GGMessage for ServerPlayerDeath {
    const KIND: PacketKind = PacketKind::PlayerDeath;
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

#[derive(Debug, Clone)]
pub struct ServerFullUpdate {
    pub team_death_match_invulnerability_ticks: u16,
    pub player_info: Vec<PlayerUpdateInfo>,
    pub red_intel: Vec<RawIntel>,
    pub blu_intel: Vec<RawIntel>,
    pub cap_limit: u8,
    pub caps: Caps,
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

#[derive(Debug, Clone)]
pub struct ServerInputState {
    pub inputs: Vec<Option<RawInput>>,
}

impl GGMessage for ServerInputState {
    const KIND: PacketKind = PacketKind::InputState;
}

#[derive(Debug, Clone)]
pub struct ServerJoinUpdate {
    pub client_player_id: PlayerId,
    pub map_area: u8,
}

impl GGMessage for ServerJoinUpdate {
    const KIND: PacketKind = PacketKind::JoinUpdate;
}

#[derive(Debug, Clone)]
pub struct ServerMessageString {
    pub message: String,
}

impl GGMessage for ServerMessageString {
    const KIND: PacketKind = PacketKind::MessageString;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerChangeClass {
    pub player_index: PlayerId,
    pub player_class: ClassGeneric,
}

impl GGMessage for ServerPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerChangeTeam {
    pub player_index: PlayerId,
    pub player_team: Team,
}

impl GGMessage for ServerPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerJoin {
    pub player_name: String,
}

impl GGMessage for ServerPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerLeave {
    pub player_index: PlayerId,
}

impl GGMessage for ServerPlayerLeave {
    const KIND: PacketKind = PacketKind::PlayerLeave;
}

#[derive(Debug, Clone)]
pub struct ServerPlayerSpawn {
    pub player_index: PlayerId,
    pub spawn_index: u8,
    pub spawn_group: u8,
}

impl GGMessage for ServerPlayerSpawn {
    const KIND: PacketKind = PacketKind::PlayerSpawn;
}

#[derive(Debug, Clone)]
pub struct ServerQuickUpdate {
    pub player_characters: Vec<Option<(RawInput, RawPlayerInfo)>>,
}

impl GGMessage for ServerQuickUpdate {
    const KIND: PacketKind = PacketKind::QuickUpdate;
}

#[derive(Debug, Clone)]
pub struct ServerReserveSlot;

impl GGMessage for ServerReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}

#[derive(Debug, Clone)]
pub struct ServerServerFull;

impl GGMessage for ServerServerFull {
    const KIND: PacketKind = PacketKind::ServerFull;
}
