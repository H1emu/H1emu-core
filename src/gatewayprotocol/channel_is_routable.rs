use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::protocol::GatewayOpcode;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelIsRoutablePacket {
    pub is_routable: bool,
}
#[wasm_bindgen]
impl ChannelIsRoutablePacket {
    #[wasm_bindgen(constructor)]
    pub fn new(is_routable: bool) -> ChannelIsRoutablePacket {
        ChannelIsRoutablePacket { is_routable }
    }
    pub fn get_is_routable(&self) -> bool {
        self.is_routable
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::ChannelIsRoutable as u8)
            .unwrap_or_default();
        wtr
    }
}

impl ChannelIsRoutablePacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> ChannelIsRoutablePacket {
        let is_routable = rdr.read_u8().unwrap_or_default() == 1;
        ChannelIsRoutablePacket { is_routable }
    }
}
