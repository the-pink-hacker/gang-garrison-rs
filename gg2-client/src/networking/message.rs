use gg2_common::{
    error::Result, gamemode::Gamemode, networking::message::MessageWriter, string::GGStringShort,
};

pub mod client;
pub mod server;

#[allow(async_fn_in_trait)]
pub trait ClientNetworkSerialize: Sized {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()>;
}

#[allow(async_fn_in_trait)]
pub trait ClientNetworkDeserialize: Sized {
    async fn deserialize<I, C>(payload: &mut I, context: &C) -> Result<Self>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext;
}

#[allow(async_fn_in_trait)]
pub trait ClientNetworkDeserializationContext {
    async fn current_map_gamemode(&self) -> Result<Gamemode>;
}

impl ClientNetworkSerialize for &GGStringShort {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_utf8_short_string(self);

        Ok(())
    }
}
