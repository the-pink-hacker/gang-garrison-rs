use gg2_common::{error::Result, networking::message::*};

use super::ClientNetworkSerialize;

impl ClientNetworkSerialize for ClientHello {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        let protocol_bytes = self.protocol.into_bytes();
        buffer.extend(protocol_bytes.iter());

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeClass {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.class as u8);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeTeam {
    async fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.team as u8);

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
