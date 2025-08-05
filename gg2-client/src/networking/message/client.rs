use gg2_common::{error::Result, networking::message::*, player::RawInput};

use super::ClientNetworkSerialize;

macro_rules! generic_message {
    ($name: ident {$($case: ident),+$(,)?}) => {
        impl ClientNetworkSerialize for $name {
            async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
                match self {
                    $(Self::$case(message) => (message.serialize(buffer).await?)),+,
                }

                Ok(())
            }
        }
    };
}

generic_message!(ClientMessageGeneric {
    Hello,
    InputState,
    PlayerChangeClass,
    PlayerChangeTeam,
    PlayerJoin,
    ReserveSlot,
});

impl ClientNetworkSerialize for ClientHello {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        let protocol_bytes = self.protocol.into_bytes();
        buffer.extend(protocol_bytes.iter());

        Ok(())
    }
}

impl ClientNetworkSerialize for RawInput {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_u8(self.key_state.into());
        buffer.write_u16(self.aim_direction);
        buffer.write_fixed_point_u16(self.aim_distance, 2.0);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientInputState {
    #[inline]
    fn serialize(self, buffer: &mut Vec<u8>) -> impl Future<Output = Result<()>> {
        self.input.serialize(buffer)
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeClass {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_u8(self.class as u8);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeTeam {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_u8(self.team as u8);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerJoin {
    async fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        Ok(())
    }
}

impl ClientNetworkSerialize for ClientReserveSlot {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_utf8_short_string(&self.player_name);

        Ok(())
    }
}
