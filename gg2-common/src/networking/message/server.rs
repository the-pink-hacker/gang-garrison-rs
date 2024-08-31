use crate::networking::{error::Result, PacketKind};

use super::{read_utf8_short_string, GGMessage};

#[derive(Debug)]
pub struct ServerHello {
    pub server_name: String,
    pub map_name: String,
    pub map_md5: Option<u128>,
    pub plugins: Vec<()>,
}

impl GGMessage for ServerHello {
    const KIND: PacketKind = PacketKind::Hello;

    fn serialize(self, _buffer: &mut Vec<u8>) -> Result<()> {
        unimplemented!();
    }

    fn deserialize<I: IntoIterator<Item = u8>>(payload: I) -> Result<Self> {
        let mut payload = payload.into_iter();
        let server_name = read_utf8_short_string(&mut payload)?;
        let map_name = read_utf8_short_string(&mut payload)?;

        // TODO: Parse MD5 and plugins
        //let md5_string = read_utf8_short_string(&mut payload)?;

        Ok(Self {
            server_name,
            map_name,
            map_md5: None,
            plugins: Vec::new(),
        })
    }
}
