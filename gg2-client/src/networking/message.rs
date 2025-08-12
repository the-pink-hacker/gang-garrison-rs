use gg2_common::{
    error::Result,
    networking::{PacketKind, message::MessageWriter},
    string::GGStringShort,
};

pub mod client;
pub mod server;

pub trait ClientNetworkSerialize: Sized {
    fn serialize(self, buffer: &mut Vec<u8>) -> impl Future<Output = Result<()>>;
}

pub trait ClientNetworkDeserialize: Sized {
    fn deserialize<I, C>(payload: &mut I, context: &C) -> impl Future<Output = Result<Self>>
    where
        I: Iterator<Item = u8>,
        C: ClientNetworkDeserializationContext;
}

pub trait ClientNetworkDeserializationContext {
    fn players_length(&self) -> impl Future<Output = u8>;

    fn deserialize_gamemode_state<I>(
        &self,
        payload: &mut I,
        kind: PacketKind,
    ) -> impl Future<Output = Result<()>>
    where
        I: Iterator<Item = u8>;

    fn current_map_control_points_length(&self) -> impl Future<Output = Result<u8>>;
}

impl ClientNetworkSerialize for &GGStringShort {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_utf8_short_string(self);

        Ok(())
    }
}
