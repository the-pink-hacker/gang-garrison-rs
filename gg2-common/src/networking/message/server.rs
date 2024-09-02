use crate::networking::{
    error::{Error, Result},
    PacketKind,
};

use super::{read_utf8_long_string, read_utf8_short_string, GGMessage};

#[derive(Debug)]
pub struct ServerHello {
    pub server_name: String,
    pub map_name: String,
    pub map_md5: Option<u128>,
    pub plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let server_name = read_utf8_short_string(payload)?;
        let map_name = read_utf8_short_string(payload)?;

        let map_md5_string = read_utf8_short_string(payload)?;
        let map_md5 = if map_md5_string.is_empty() {
            None
        } else {
            Some(map_md5_string.parse().map_err(|_| Error::PacketPayload)?)
        };

        let plugins_amounts = payload.next().ok_or(Error::UnexpectedEOF)?;
        let plugins_raw = read_utf8_long_string(payload)?;
        println!("Found {} plugins: [ {} ]", plugins_amounts, plugins_raw);

        Ok(Self {
            server_name,
            map_name,
            map_md5,
            plugins: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct ServerReserveSlot;

impl GGMessage for ServerReserveSlot {
    const KIND: PacketKind = PacketKind::ReserveSlot;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(ServerReserveSlot)
    }
}

#[derive(Debug)]
pub struct ServerServerFull;

impl GGMessage for ServerServerFull {
    const KIND: PacketKind = PacketKind::ServerFull;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(_payload: &mut I) -> Result<Self> {
        Ok(ServerServerFull)
    }
}

// TODO: Implement inputstate
#[derive(Debug)]
pub struct ServerInputstate;

impl GGMessage for ServerInputstate {
    const KIND: PacketKind = PacketKind::Inputstate;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        Ok(ServerInputstate {})
    }
}

// TODO: Implement quick update
#[derive(Debug)]
pub struct ServerQuickUpdate;

impl GGMessage for ServerQuickUpdate {
    const KIND: PacketKind = PacketKind::QuickUpdate;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        Ok(ServerQuickUpdate {})
    }
}

#[derive(Debug)]
pub struct ServerPlayerJoin {
    pub player_name: String,
}

impl GGMessage for ServerPlayerJoin {
    const KIND: PacketKind = PacketKind::PlayerJoin;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_name = read_utf8_short_string(payload)?;

        Ok(ServerPlayerJoin { player_name })
    }
}

#[derive(Debug)]
pub struct ServerJoinUpdate {
    pub amount_of_players: u8,
    pub map_area: u8,
}

impl GGMessage for ServerJoinUpdate {
    const KIND: PacketKind = PacketKind::JoinUpdate;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let amount_of_players = payload.next().ok_or(Error::UnexpectedEOF)?;
        let map_area = payload.next().ok_or(Error::UnexpectedEOF)?;
        Ok(ServerJoinUpdate {
            amount_of_players,
            map_area,
        })
    }
}
