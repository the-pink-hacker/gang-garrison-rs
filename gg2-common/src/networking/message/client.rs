use uuid::Uuid;

use crate::{
    networking::{error::Result, PacketKind, PROTOCOL_UUID},
    player::{class::ClassGeneric, team::Team},
};

use super::{write_utf8_short_string, GGMessage, NetworkSerialize};

pub struct ClientHello {
    pub protocol: Uuid,
}

impl GGMessage for ClientHello {
    const KIND: PacketKind = PacketKind::Hello;
}

impl NetworkSerialize for ClientHello {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        let protocol_bytes = self.protocol.into_bytes();
        buffer.extend(protocol_bytes.iter());
        Ok(())
    }
}

impl Default for ClientHello {
    fn default() -> Self {
        Self {
            protocol: PROTOCOL_UUID,
        }
    }
}

#[derive(Debug)]
pub struct ClientReserveSlot {
    pub player_name: String,
}

impl GGMessage for ClientReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;
}

impl NetworkSerialize for ClientReserveSlot {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        write_utf8_short_string(self.player_name, buffer)
    }
}

#[derive(Debug)]
pub struct ClientPlayerJoin;

impl GGMessage for ClientPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;
}

impl NetworkSerialize for ClientPlayerJoin {
    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientPlayerChangeClass {
    pub class: ClassGeneric,
}

impl GGMessage for ClientPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;
}

impl NetworkSerialize for ClientPlayerChangeClass {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.class as u8);

        Ok(())
    }
}

#[derive(Debug)]
pub struct ClientPlayerChangeTeam {
    pub team: Team,
}

impl GGMessage for ClientPlayerChangeTeam {
    const KIND: PacketKind = PacketKind::PlayerChangeTeam;
}

impl NetworkSerialize for ClientPlayerChangeTeam {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.team as u8);

        Ok(())
    }
}
