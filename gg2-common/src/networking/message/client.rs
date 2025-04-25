use uuid::Uuid;

use crate::{
    networking::{GGMessage, PROTOCOL_UUID, PacketKind},
    player::{class::ClassGeneric, team::Team},
};

use super::GGStringShort;

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

#[derive(Debug)]
pub struct ClientPlayerChangeClass {
    pub class: ClassGeneric,
}

impl GGMessage for ClientPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

#[derive(Debug)]
pub struct ClientPlayerChangeTeam {
    pub team: Team,
}

impl GGMessage for ClientPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

#[derive(Debug)]
pub struct ClientPlayerJoin;

impl GGMessage for ClientPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

#[derive(Debug)]
pub struct ClientReserveSlot {
    pub player_name: GGStringShort,
}

impl GGMessage for ClientReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}
