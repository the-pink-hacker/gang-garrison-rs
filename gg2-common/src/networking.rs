use message::GGMessage;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use uuid::{Uuid, uuid};

pub mod error;
pub mod message;

/// The protocol UUID that is sent on a client to server Hello message
pub const PROTOCOL_UUID: Uuid = uuid!("b31c2209-4256-9a19-d0ef-c71c5373bd75");
/// The default GG2 server port
pub const DEFAULT_PORT: u16 = 8190;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketKind {
    Hello = 0,
    PlayerJoin = 1,
    PlayerLeave = 2,
    PlayerChangeTeam = 3,
    PlayerChangeClass = 4,
    PlayerSpawn = 5,
    InputState = 6,
    ChangeMap = 7,
    FullUpdate = 8,
    QuickUpdate = 9,
    PlayerDeath = 10,
    ServerFull = 11,
    RedTeamCap = 12,
    BlueTeamCap = 13,
    MapEnd = 14,
    ChatBubble = 15,
    BuildSentry = 16,
    DestroySentry = 17,
    Balance = 18,
    GrabIntel = 19,
    ScoreIntel = 20,
    DropIntel = 21,
    UberCharged = 22,
    Uber = 23,
    Omnom = 24,
    PasswordRequest = 25,
    PasswordWrong = 27,
    CaptureUpdate = 28,
    CpCaptured = 30,
    PlayerChangeName = 31,
    GeneratorDestroy = 32,
    ArenaWaitForPlayers = 33,
    ArenaEndround = 34,
    ArenaRestart = 35,
    UnlockCp = 36,
    ServerKick = 37,
    Kick = 38,
    KickName = 39,
    ArenaStartround = 40,
    ToggleZoom = 41,
    ReturnIntel = 42,
    IncompatibleProtocol = 43,
    JoinUpdate = 44,
    DownloadMap = 45,
    SentryPosition = 46,
    RewardUpdate = 47,
    RewardRequest = 50,
    RewardChallengeCode = 51,
    RewardChallengeResponse = 52,
    MessageString = 53,
    WeaponFire = 54,
    PluginPacket = 55,
    KickBadPluginPacket = 56,
    Ping = 57,
    ClientSettings = 58,
    KickMultiClient = 59,
    ReserveSlot = 60,
}

pub trait AsPacketKind {
    fn as_packet_kind(&self) -> PacketKind;
}

impl<T: GGMessage> AsPacketKind for T {
    #[inline]
    fn as_packet_kind(&self) -> PacketKind {
        T::KIND
    }
}

impl<T: AsPacketKind> From<T> for PacketKind {
    #[inline]
    fn from(value: T) -> Self {
        value.as_packet_kind()
    }
}
