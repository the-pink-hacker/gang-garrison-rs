use gg2_common::{
    networking::{error::*, message::MessageWriter},
    string::GGStringShort,
};

pub mod client;
pub mod server;

pub trait ClientNetworkSerialize: Sized {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;
}

pub trait ClientNetworkDeserialize: Sized {
    fn deserialize<I: Iterator<Item = u8>>(payload: &mut I) -> Result<Self>;
}

impl ClientNetworkSerialize for &GGStringShort {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_utf8_short_string(self);

        Ok(())
    }
}
