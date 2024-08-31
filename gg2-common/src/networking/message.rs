use crate::networking::error::Error;

use super::{error::Result, NetworkPacket, PacketKind};
pub use client::*;
pub use server::*;

mod client;
mod server;

pub trait GGMessage: Sync + Send + Sized {
    const KIND: PacketKind;

    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;

    fn deserialize<I: Iterator<Item = u8>>(payload: I) -> Result<Self>;

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

fn write_utf8_short_string(text: String, buffer: &mut Vec<u8>) -> Result<()> {
    let length = text.len().try_into().map_err(Error::StringLength)?;
    buffer.push(length);
    buffer.extend(text.bytes());
    Ok(())
}
