use crate::{
    chat::bubble::ChatBubble,
    damage::source::DamageSource,
    game::intel::RawIntel,
    hud::GamemodeHud,
    networking::PacketKind,
    player::{
        PlayerId, RawAdditionalPlayerInfo, RawInput, RawPlayerInfo,
        class::ClassGeneric,
        team::{Captures, Team, TeamSpawnable},
    },
};
use glam::Vec2;

use super::{GGMessage, GGStringLong, GGStringShort};

macro_rules! generic_message {
    ($name:ident {$($case:ident),+$(,)?}) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($case(${concat(Server, $case)})),+,
        }

        impl From<ServerMessageGeneric> for PacketKind {
            fn from(value: ServerMessageGeneric) -> Self {
                match value {
                    $(ServerMessageGeneric::$case(_) => PacketKind::$case),+,
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

/// Updates the client about captures
#[derive(Debug, Clone)]
pub struct ServerCaptureUpdate {
    /// The amount of players on the server
    pub player_amount: u8,
    /// The server's current captures
    pub captures: Captures,
    /// The hud specific to the current gamemode
    pub hud: GamemodeHud,
}

impl GGMessage for ServerCaptureUpdate {
    const KIND: PacketKind = PacketKind::CaptureUpdate;
}

/// The server is changing maps
#[derive(Debug, Clone)]
pub struct ServerChangeMap {
    /// The new map
    pub map_name: GGStringShort,
    /// The new map's MD5 hash
    /// Isn't present if the map is builtin
    pub map_md5: Option<u128>,
}

impl GGMessage for ServerChangeMap {
    const KIND: PacketKind = PacketKind::ChangeMap;
}

#[derive(Debug, Clone)]
pub struct ServerChatBubble {
    pub bubble: ChatBubble,
}

impl GGMessage for ServerChatBubble {
    const KIND: PacketKind = PacketKind::ChatBubble;
}

/// Intel was dropped by a player
/// Implicitly happens on player death
#[derive(Debug, Clone)]
pub struct ServerDropIntel {
    /// The player who dropped the intel
    pub player_id: PlayerId,
}

impl GGMessage for ServerDropIntel {
    const KIND: PacketKind = PacketKind::DropIntel;
}

/// A player picked up intel
#[derive(Debug, Clone)]
pub struct ServerGrabIntel {
    /// The player that grabbed the intel
    pub player_id: PlayerId,
}

impl GGMessage for ServerGrabIntel {
    const KIND: PacketKind = PacketKind::GrabIntel;
}

/// A player has been killed
#[derive(Debug, Clone)]
pub struct ServerPlayerDeath {
    /// The player that died
    pub target: PlayerId,
    /// If the target was killed by a player, the player that killed
    pub attacker: Option<PlayerId>,
    /// If the kill was helped by another player, the player that helped
    pub assist: Option<PlayerId>,
    /// What damage caused the target to die
    pub damage_source: DamageSource,
}

impl GGMessage for ServerPlayerDeath {
    const KIND: PacketKind = PacketKind::PlayerDeath;
}

/// Stats about the player and optionally the character
#[derive(Debug, Clone)]
pub struct PlayerUpdateInfo {
    /// How many kills the player has
    pub kills: u8,
    /// How deaths the player has
    pub deaths: u8,
    /// How many captures the player has
    pub captures: u8,
    /// How many assists the player has
    pub assists: u8,
    pub destruction: u8,
    /// How many stabs the player has
    pub stabs: u8,
    pub healing: u16,
    pub defenses: u8,
    /// Wether the player invulnerable
    pub invulnerability: bool,
    pub bonus: u8,
    /// The player's score
    pub points: u8,
    /// Wether the player has queue jumping enabled
    pub queue_jump: bool,
    pub rewards: GGStringLong,
    pub dominations: Vec<u8>,
    /// The player's charater if they're spawned in the game
    pub character: Option<(RawInput, RawPlayerInfo, RawAdditionalPlayerInfo)>,
}

/// Update to inform a client about everything at once
#[derive(Debug, Clone)]
pub struct ServerFullUpdate {
    pub team_death_match_invulnerability_ticks: u16,
    /// A list of all player's update info in ID order
    pub player_info: Vec<PlayerUpdateInfo>,
    /// All red intel currently spawned
    pub red_intel: Vec<RawIntel>,
    /// All blu intel currently spawned
    pub blu_intel: Vec<RawIntel>,
    /// The max number of captures allowed
    pub capture_limit: u8,
    pub captures: Captures,
    /// The hud specific to the current gamemode
    pub hud: GamemodeHud,
    /// Scout class limit
    pub scout_limit: u8,
    /// Soldier class limit
    pub soldier_limit: u8,
    /// Sniper class limit
    pub sniper_limit: u8,
    /// Demoman class limit
    pub demoman_limit: u8,
    /// Medic class limit
    pub medic_limit: u8,
    /// Engineer class limit
    pub engineer_limit: u8,
    /// Heavy class limit
    pub heavy_limit: u8,
    /// Spy class limit
    pub spy_limit: u8,
    /// Pyro class limit
    pub pyro_limit: u8,
    /// Quote class limit
    pub quote_limit: u8,
}

impl GGMessage for ServerFullUpdate {
    const KIND: PacketKind = PacketKind::FullUpdate;
}

/// Used to retreive infomation on the server
#[derive(Debug, Clone)]
pub struct ServerHello {
    /// The server's name
    pub server_name: GGStringShort,
    /// The server's map name
    pub map_name: GGStringShort,
    /// A MD5 hash of the map
    /// Not present when the map is builtin
    pub map_md5: Option<u128>,
    // TODO: Implement plugin parsing
    pub plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;
}

/// The server doesn't support the client's network protocol
#[derive(Debug, Clone)]
pub struct ServerIncompatibleProtocol;

impl GGMessage for ServerIncompatibleProtocol {
    const KIND: PacketKind = PacketKind::IncompatibleProtocol;
}

/// The inputs of all players
#[derive(Debug, Clone)]
pub struct ServerInputState {
    /// A list of all player's inputs in ID
    /// None if player doesn't have a character
    pub inputs: Vec<Option<RawInput>>,
}

impl GGMessage for ServerInputState {
    const KIND: PacketKind = PacketKind::InputState;
}

/// Update when the player first joins the server
#[derive(Debug, Clone)]
pub struct ServerJoinUpdate {
    /// The player id the client will have
    pub client_player_id: PlayerId,
    /// I have no fucking idea
    pub map_area: u8,
}

impl GGMessage for ServerJoinUpdate {
    const KIND: PacketKind = PacketKind::JoinUpdate;
}

/// The server sent a message
#[derive(Debug, Clone)]
pub struct ServerMessageString {
    /// The server's message
    pub message: GGStringShort,
}

impl GGMessage for ServerMessageString {
    const KIND: PacketKind = PacketKind::MessageString;
}

/// Heavy used the Omnom special ability
#[derive(Debug, Clone)]
pub struct ServerOmnom;

impl GGMessage for ServerOmnom {
    const KIND: PacketKind = PacketKind::Omnom;
}

/// The server is requesting a password
#[derive(Debug, Clone, Default)]
pub struct ServerPasswordRequest;

impl GGMessage for ServerPasswordRequest {
    const KIND: PacketKind = PacketKind::PasswordRequest;
}

/// The password sent to the server was wrong
#[derive(Debug, Clone, Default)]
pub struct ServerPasswordWrong;

impl GGMessage for ServerPasswordWrong {
    const KIND: PacketKind = PacketKind::PasswordWrong;
}

/// A player is changing classes
#[derive(Debug, Clone)]
pub struct ServerPlayerChangeClass {
    /// The player changing their class
    pub player_id: PlayerId,
    /// The player's new class
    pub player_class: ClassGeneric,
}

impl GGMessage for ServerPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

/// A player is changing their name
#[derive(Debug, Clone)]
pub struct ServerPlayerChangeName {
    /// The player changing their name
    pub player_id: PlayerId,
    /// The player's new name
    pub name: GGStringShort,
}

/// A player is changing their team
#[derive(Debug, Clone)]
pub struct ServerPlayerChangeTeam {
    /// The player changing teams
    pub player_id: PlayerId,
    /// The player's new team
    pub player_team: Team,
}

impl GGMessage for ServerPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

/// A player has joined the lobby
#[derive(Debug, Clone)]
pub struct ServerPlayerJoin {
    /// The name of the player that joined
    pub player_name: GGStringShort,
}

impl GGMessage for ServerPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

/// A player has left the lobby
#[derive(Debug, Clone)]
pub struct ServerPlayerLeave {
    /// The player that left
    pub player_id: PlayerId,
}

impl GGMessage for ServerPlayerLeave {
    const KIND: PacketKind = PacketKind::PlayerLeave;
}

/// A player spawned in the world
#[derive(Debug, Clone)]
pub struct ServerPlayerSpawn {
    /// The player that spawned
    pub player_id: PlayerId,
    /// The index of the spawnpoint in a group
    pub spawn_index: u8,
    /// The spawnpoint group
    pub spawn_group: u8,
}

impl GGMessage for ServerPlayerSpawn {
    const KIND: PacketKind = PacketKind::PlayerSpawn;
}

/// Update a client with little information
#[derive(Debug, Clone)]
pub struct ServerQuickUpdate {
    /// A list of all player characters in ID order
    pub player_characters: Vec<Option<(RawInput, RawPlayerInfo)>>,
}

impl GGMessage for ServerQuickUpdate {
    const KIND: PacketKind = PacketKind::QuickUpdate;
}

/// A confirmation that the player has reserved a slot
#[derive(Debug, Clone)]
pub struct ServerReserveSlot;

impl GGMessage for ServerReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}

/// An intel was returned
#[derive(Debug, Clone)]
pub struct ServerReturnIntel {
    /// The intel's team
    pub team: TeamSpawnable,
}

impl GGMessage for ServerReturnIntel {
    const KIND: PacketKind = PacketKind::ReturnIntel;
}

/// A player scored intel
#[derive(Debug, Clone)]
pub struct ServerScoreIntel {
    /// The player that scored the intel
    pub player_id: PlayerId,
}

impl GGMessage for ServerScoreIntel {
    const KIND: PacketKind = PacketKind::ScoreIntel;
}

/// The server isn't accepting more players
#[derive(Debug, Clone)]
pub struct ServerServerFull;

impl GGMessage for ServerServerFull {
    const KIND: PacketKind = PacketKind::ServerFull;
}

/// A player fired a weapon
#[derive(Debug, Clone)]
pub struct ServerWeaponFire {
    /// The player who fired; must have a character
    pub player_id: PlayerId,
    /// The attacker's position
    /// 16-bit fixed point with a scale of 5
    pub position: Vec2,
    /// The attacker's velocity
    /// 8-bit fixed point with a scale of 8.5
    pub velocity: Vec2,
    /// Seed used for RNG
    pub seed: u16,
}

impl GGMessage for ServerWeaponFire {
    const KIND: PacketKind = PacketKind::WeaponFire;
}
