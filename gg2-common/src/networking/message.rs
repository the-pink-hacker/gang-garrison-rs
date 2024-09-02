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
    let length = stream.next().ok_or(Error::UnexpectedEOF)?;
    match length {
        0 => Ok(None),
        // The length of a 128-bit hex string
        32 => {
            let hex_bytes = stream.take(32).collect();
            let hex_string = String::from_utf8(hex_bytes).map_err(|_| Error::PacketPayload)?;
            let hash_bytes = hex::decode(hex_string).map_err(|_| Error::PacketPayload)?;

            assert_eq!(hash_bytes.len(), 16);

            Ok(Some(
                hash_bytes
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, byte)| (byte as u128) << (i * 8))
                    .sum(),
            ))
        }
        _ => Err(Error::PacketPayload),
    }
}

fn write_utf8_short_string(text: String, buffer: &mut Vec<u8>) -> Result<()> {
    let bytes = text.bytes();
    let length = bytes.len().try_into().map_err(Error::StringLength)?;
    buffer.push(length);
    buffer.extend(bytes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string_short() {
        let mut data = [4, b't', b'e', b's', b't'].into_iter();
        let parsed = read_utf8_short_string(&mut data).unwrap();
        assert_eq!(parsed, "test");
    }

    #[test]
    fn read_string_long() {
        let mut data = [4, 0, b'l', b'o', b'n', b'g'].into_iter();
        let parsed = read_utf8_long_string(&mut data).unwrap();
        assert_eq!(parsed, "long");
    }

    #[test]
    fn read_md5_string() {
        let mut data = vec![32];
        data.extend("e0cae13971b1ba6a8eef49cbcfc944bf".as_bytes());
        let parsed = read_md5(&mut data.into_iter()).unwrap();
        assert_eq!(parsed, Some(298800483114597941956032572434422514879));
    }

    #[test]
    fn read_md5_empty() {
        let mut data = vec![0].into_iter();
        let parsed = read_md5(&mut data).unwrap();
        assert_eq!(parsed, None);
    }
}
