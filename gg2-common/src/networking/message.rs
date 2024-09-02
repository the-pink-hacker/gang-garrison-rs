use crate::networking::error::Error;

use super::{error::Result, NetworkPacket, PacketKind};
pub use client::*;
pub use server::*;

mod client;
mod server;

pub trait GGMessage: Sync + Send + Sized {
    const KIND: PacketKind;

    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;

    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self>;

    fn into_packet(self) -> Result<NetworkPacket> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(NetworkPacket {
            kind: Self::KIND,
            data,
        })
    }
}

fn read_utf8_short_string<I: Iterator<Item = u8>>(stream: &mut I) -> Result<String> {
    let length = stream.next().ok_or(Error::UnexpectedEOF)? as usize;
    let bytes = stream.take(length).collect();
    String::from_utf8(bytes).map_err(|_| Error::PacketPayload)
}

fn read_u16<I: Iterator<Item = u8>>(stream: &mut I) -> Result<u16> {
    let least_significant = stream.next().ok_or(Error::UnexpectedEOF)? as u16;
    let most_significant = stream.next().ok_or(Error::UnexpectedEOF)? as u16;
    Ok(least_significant | (most_significant << 8))
}

fn read_utf8_long_string<I: Iterator<Item = u8>>(stream: &mut I) -> Result<String> {
    let length = read_u16(stream)? as usize;
    let bytes = stream.take(length).collect();
    String::from_utf8(bytes).map_err(|_| Error::PacketPayload)
}

fn read_md5<I: Iterator<Item = u8>>(stream: &mut I) -> Result<Option<u128>> {
    let md5_string = read_utf8_short_string(stream)?;
    if md5_string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(md5_string.parse().map_err(|_| Error::PacketPayload)?))
    }
}

fn write_utf8_short_string(text: String, buffer: &mut Vec<u8>) -> Result<()> {
    let length = text.len().try_into().map_err(Error::StringLength)?;
    buffer.push(length);
    buffer.extend(text.bytes());
    Ok(())
}
