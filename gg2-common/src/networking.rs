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
    Hello = 0,
    PlayerJoin = 1,
    PlayerLeave = 2,
    PlayerChangeteam = 3,
    PlayerChangeclass = 4,
    PlayerSpawn = 5,
    Inputstate = 6,
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
    Omnomnomnom = 24,
    PasswordRequest = 25,
    PasswordWrong = 27,
    CapsUpdate = 28,
    CpCaptured = 30,
    PlayerChangename = 31,
    GeneratorDestroy = 32,
    ArenaWaitForPlayers = 33,
    ArenaEndround = 34,
    ArenaRestart = 35,
    Unlockcp = 36,
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
