use uuid::Uuid;

use crate::networking::error::Error;

use super::{error::Result, NetworkPacket, PacketKind, PROTOCOL_UUID};

pub trait GGMessage: Sync + Send {
    const KIND: PacketKind;

    fn serialize(&self) -> &[u8];

    fn deserialize<I: IntoIterator<Item = u8>>(payload: I) -> Result<Self>
    where
        Self: Sized;
}

impl<T: GGMessage> From<T> for NetworkPacket {
    fn from(value: T) -> Self {
        Self {
            kind: T::KIND,
            data: value.serialize().to_vec(),
        }
    }
}

pub struct ClientHello {
    protocol: Uuid,
}

impl GGMessage for ClientHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(&self) -> &[u8] {
        self.protocol.as_bytes()
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
    server_name: String,
    map_name: String,
    map_md5: Option<u128>,
    plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(&self) -> &[u8] {
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
