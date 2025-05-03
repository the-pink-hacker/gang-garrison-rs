use gg2_common::networking::{error::Result, message::*};

use super::ClientNetworkSerialize;

impl ClientNetworkSerialize for ClientHello {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        let protocol_bytes = self.protocol.into_bytes();
        buffer.extend(protocol_bytes.iter());

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeClass {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.class as u8);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerChangeTeam {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.team as u8);

        Ok(())
    }
}

impl ClientNetworkSerialize for ClientPlayerJoin {
    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        Ok(())
    }
}

impl ClientNetworkSerialize for ClientReserveSlot {
    fn serialize(self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_utf8_short_string(&self.player_name);

        Ok(())
    }
}
