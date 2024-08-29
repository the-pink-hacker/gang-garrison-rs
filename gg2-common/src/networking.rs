use error::Error;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use uuid::{uuid, Uuid};

pub mod error;
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

impl TryFrom<&Vec<u8>> for NetworkPacket {
    type Error = error::Error;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut stream = value.iter().cloned();
        let raw_kind = stream.next().ok_or(error::Error::PacketEmpty)?;
        let kind = raw_kind
            .try_into()
            .map_err(|_| Error::PacketKind(raw_kind))?;
        let data = stream.collect();
        Ok(Self { kind, data })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
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
