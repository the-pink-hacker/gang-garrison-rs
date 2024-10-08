use bevy::prelude::*;

use crate::networking::error::Error;

use super::{error::Result, PacketKind};
pub use client::*;
pub use server::*;

mod client;
mod server;

pub trait GGMessage: Sync + Send {
    const KIND: PacketKind;
}

pub trait NetworkSerialize: Sized {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;
}

pub trait NetworkDeserialize: Sized {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self>;
}

pub trait MessageReader {
    fn read_u8(&mut self) -> Result<u8>;

    fn read_u16(&mut self) -> Result<u16>;

    fn read_bool(&mut self) -> Result<bool>;

    fn read_fixed_point_u8(&mut self, scale: f32) -> Result<f32>;

    fn read_fixed_point_u16(&mut self, scale: f32) -> Result<f32>;

    fn read_fixed_point_u16_vec2(&mut self, scale: f32) -> Result<Vec2>;

    fn read_utf8_short_string(&mut self) -> Result<String>;

    fn read_utf8_long_string(&mut self) -> Result<String>;

    fn read_md5(&mut self) -> Result<Option<u128>>;
}

impl<T> MessageReader for T
where
    T: Iterator<Item = u8>,
{
    fn read_u8(&mut self) -> Result<u8> {
        self.next().ok_or(Error::UnexpectedEOF)
    }

    fn read_u16(&mut self) -> Result<u16> {
        let least_significant = self.read_u8()? as u16;
        let most_significant = self.read_u8()? as u16;
        Ok(least_significant | (most_significant << 8))
    }

    fn read_bool(&mut self) -> Result<bool> {
        self.read_u8().and_then(|value| match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::PacketPayload),
        })
    }

    fn read_fixed_point_u8(&mut self, scale: f32) -> Result<f32> {
        self.read_u8().map(|value| value as f32 / scale)
    }

    fn read_fixed_point_u16(&mut self, scale: f32) -> Result<f32> {
        self.read_u16().map(|value| value as f32 / scale)
    }

    fn read_fixed_point_u16_vec2(&mut self, scale: f32) -> Result<Vec2> {
        let x = self.read_fixed_point_u16(scale)?;
        let y = self.read_fixed_point_u16(scale)?;
        Ok(Vec2::new(x, y))
    }

    fn read_utf8_short_string(&mut self) -> Result<String> {
        let length = self.read_u8()? as usize;
        let bytes = self.take(length).collect();
        String::from_utf8(bytes).map_err(|_| Error::PacketPayload)
    }

    fn read_utf8_long_string(&mut self) -> Result<String> {
        let length = self.read_u16()? as usize;
        let bytes = self.take(length).collect();
        String::from_utf8(bytes).map_err(|_| Error::PacketPayload)
    }

    fn read_md5(&mut self) -> Result<Option<u128>> {
        let length = self.read_u8()?;
        match length {
            0 => Ok(None),
            // The length of a 128-bit hex string
            32 => {
                let hex_bytes = self.take(32).collect();
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
        let parsed = data.read_utf8_short_string().unwrap();
        assert_eq!(parsed, "test");
    }

    #[test]
    fn read_string_long() {
        let mut data = [4, 0, b'l', b'o', b'n', b'g'].into_iter();
        let parsed = data.read_utf8_long_string().unwrap();
        assert_eq!(parsed, "long");
    }

    #[test]
    fn read_md5_string() {
        let mut data = vec![32];
        data.extend("e0cae13971b1ba6a8eef49cbcfc944bf".as_bytes());
        let parsed = data.into_iter().read_md5().unwrap();
        assert_eq!(parsed, Some(298800483114597941956032572434422514879));
    }

    #[test]
    fn read_md5_empty() {
        let mut data = vec![0].into_iter();
        let parsed = data.read_md5().unwrap();
        assert_eq!(parsed, None);
    }
}
