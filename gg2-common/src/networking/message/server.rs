use crate::{
    networking::{
        error::{Error, Result},
        message::read_md5,
        PacketKind,
    },
    player::Class,
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

        let map_md5 = read_md5(payload)?;

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
pub struct ServerInputState;

impl GGMessage for ServerInputState {
    const KIND: PacketKind = PacketKind::InputState;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        Ok(ServerInputState {})
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

#[derive(Debug)]
pub struct ServerChangeMap {
    pub map_name: String,
    pub map_md5: Option<u128>,
}

impl GGMessage for ServerChangeMap {
    const KIND: PacketKind = PacketKind::ChangeMap;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let map_name = read_utf8_short_string(payload)?;
        let map_md5 = read_md5(payload)?;
        Ok(Self { map_name, map_md5 })
    }
}

#[derive(Debug)]
pub struct ServerPlayerChangeClass {
    pub player_index: u8,
    pub player_class: Class,
}

impl GGMessage for ServerPlayerChangeClass {
    const KIND: PacketKind = PacketKind::PlayerChangeClass;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self> {
        let player_index = payload.next().ok_or(Error::UnexpectedEOF)?;
        let player_class = payload
            .next()
            .ok_or(Error::UnexpectedEOF)?
            .try_into()
            .map_err(|_| Error::PacketPayload)?;
        Ok(Self {
            player_index,
            player_class,
        })
    }
}
