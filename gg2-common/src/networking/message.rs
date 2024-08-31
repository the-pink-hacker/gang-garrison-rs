use uuid::Uuid;

use crate::networking::error::Error;

use super::{error::Result, NetworkPacket, PacketKind, PROTOCOL_UUID};

pub trait GGMessage: Sync + Send + Sized {
    const KIND: PacketKind;

    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;

    fn deserialize<I: IntoIterator<Item = u8>>(payload: I) -> Result<Self>;

    fn into_packet(self) -> Result<NetworkPacket> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(NetworkPacket {
            kind: Self::KIND,
            data,
        })
    }
}

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

    fn deserialize<I: IntoIterator<Item = u8>>(_payload: I) -> Result<Self> {
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

    fn deserialize<I: IntoIterator<Item = u8>>(payload: I) -> Result<Self> {
        let mut payload = payload.into_iter();
        let server_name = read_utf8_short_string(&mut payload)?;
        let map_name = read_utf8_short_string(&mut payload)?;

        // TODO: Parse MD5 and plugins
        //let md5_string = read_utf8_short_string(&mut payload)?;

        Ok(Self {
            server_name,
            map_name,
            map_md5: None,
            plugins: Vec::new(),
        })
    }
}

fn read_utf8_short_string<I: Iterator<Item = u8>>(stream: &mut I) -> Result<String> {
    let length = stream.next().ok_or(Error::UnexpectedEOF)? as usize;
    let bytes = stream.take(length).collect();
    String::from_utf8(bytes).map_err(|_| Error::PacketPayload)
}

fn write_utf8_short_string(text: String, buffer: &mut Vec<u8>) -> Result<()> {
    let length = text.len().try_into().map_err(Error::StringLength)?;
    buffer.push(length);
    buffer.extend(text.bytes());
    Ok(())
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

    fn deserialize<I: IntoIterator<Item = u8>>(_payload: I) -> Result<Self>
    where
        Self: Sized,
    {
        unimplemented!();
    }
}
