use uuid::Uuid;

use crate::networking::{error::Result, PacketKind, PROTOCOL_UUID};

use super::{write_utf8_short_string, GGMessage};

pub struct ClientHello {
    pub protocol: Uuid,
}

impl GGMessage for ClientHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        let protocol_bytes = self.protocol.into_bytes();
        buffer.extend(protocol_bytes.iter());
        Ok(())
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: I) -> Result<Self> {
        unimplemented!();
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

    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        write_utf8_short_string(self.player_name, buffer)
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: I) -> Result<Self> {
        unimplemented!();
    }
}

#[derive(Debug)]
pub struct ClientPlayerJoin;

impl GGMessage for ClientPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        Ok(())
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: I) -> Result<Self> {
        unimplemented!()
    }
}
