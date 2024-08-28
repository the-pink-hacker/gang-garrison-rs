use std::fmt::{Debug, Formatter};

use uuid::{uuid, Uuid};

pub mod message;

pub const PROTOCOL_UUID: Uuid = uuid!("b31c2209-4256-9a19-d0ef-c71c5373bd75");

#[derive(Clone)]
pub struct NetworkPacket {
    pub kind: PacketKind,
    pub data: Vec<u8>,
}

impl From<NetworkPacket> for Vec<u8> {
    fn from(value: NetworkPacket) -> Self {
        let mut output = value.data;
        output.insert(0, value.kind.into());
        output
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PacketKind {
    Hello,
    PlayerJoin,
    PlayerLeave,
    PlayerChangeteam,
    PlayerChangeclass,
    PlayerSpawn,
    Inputstate,
    ChangeMap,
    FullUpdate,
    QuickUpdate,
    PlayerDeath,
    ServerFull,
    RedTeamCap,
    BlueTeamCap,
    MapEnd,
    ChatBubble,
    BuildSentry,
    DestroySentry,
    Balance,
    GrabIntel,
    ScoreIntel,
    DropIntel,
    UberCharged,
    Uber,
    Omnomnomnom,
    PasswordRequest,
    PasswordWrong,
    CapsUpdate,
    CpCaptured,
    PlayerChangename,
    GeneratorDestroy,
    ArenaWaitForPlayers,
    ArenaEndround,
    ArenaRestart,
    Unlockcp,
    ServerKick,
    Kick,
    KickName,
    ArenaStartround,
    ToggleZoom,
    ReturnIntel,
    IncompatibleProtocol,
    JoinUpdate,
    DownloadMap,
    SentryPosition,
    RewardUpdate,
    RewardRequest,
    RewardChallengeCode,
    RewardChallengeResponse,
    MessageString,
    WeaponFire,
    PluginPacket,
    KickBadPluginPacket,
    Ping,
    ClientSettings,
    KickMultiClient,
    ReserveSlot,
}

impl From<PacketKind> for u8 {
    fn from(value: PacketKind) -> Self {
        value as u8
    }
}
