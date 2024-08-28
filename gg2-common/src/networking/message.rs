use super::{NetworkPacket, PacketKind};

pub trait GGMessage {
    fn serialize(&self) -> &[u8];

    fn get_kind(&self) -> PacketKind;
}

impl<T: GGMessage> From<T> for NetworkPacket {
    fn from(value: T) -> Self {
        Self {
            kind: value.get_kind(),
            data: value.serialize().to_vec(),
        }
    }
}

pub struct GGMessageHello;

impl GGMessage for GGMessageHello {
    fn serialize(&self) -> &[u8] {
        &[]
    }

    fn get_kind(&self) -> PacketKind {
        PacketKind::Hello
    }
}
