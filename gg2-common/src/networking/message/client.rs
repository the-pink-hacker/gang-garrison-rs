use uuid::Uuid;

use crate::{
    networking::{AsPacketKind, GGMessage, PROTOCOL_UUID, PacketKind},
    player::{RawInput, class::ClassGeneric, team::Team},
};

use super::GGStringShort;

macro_rules! generic_message {
    ($name: ident {$($case: ident),+$(,)?}) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($case(${concat(Client, $case)})),+,
        }

        impl AsPacketKind for ClientMessageGeneric {
            fn as_packet_kind(&self) -> PacketKind {
                match self {
                    $(Self::$case(_) => PacketKind::$case),+,
                }
            }
        }
    };
}

generic_message!(ClientMessageGeneric {
    Hello,
    InputState,
    PlayerChangeClass,
    PlayerChangeTeam,
    PlayerJoin,
    ReserveSlot,
});

#[derive(Debug, Clone)]
pub struct ClientHello {
    pub protocol: Uuid,
}

impl GGMessage for ClientHello {
    const KIND: PacketKind = PacketKind::Hello;
}

impl Default for ClientHello {
    fn default() -> Self {
        Self {
            protocol: PROTOCOL_UUID,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientInputState {
    pub input: RawInput,
}

impl GGMessage for ClientInputState {
    const KIND: PacketKind = PacketKind::InputState;
}

#[derive(Debug, Clone)]
pub struct ClientPlayerChangeClass {
    pub class: ClassGeneric,
}

impl GGMessage for ClientPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

#[derive(Debug, Clone)]
pub struct ClientPlayerChangeTeam {
    pub team: Team,
}

impl GGMessage for ClientPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

#[derive(Debug, Clone)]
pub struct ClientPlayerJoin;

impl GGMessage for ClientPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

#[derive(Debug, Clone)]
pub struct ClientReserveSlot {
    pub player_name: GGStringShort,
}

impl GGMessage for ClientReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}
