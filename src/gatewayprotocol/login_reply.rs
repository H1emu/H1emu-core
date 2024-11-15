use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::protocol::GatewayOpcode;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginReplyPacket {
    pub logged_in: bool,
}
#[wasm_bindgen]
impl LoginReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(logged_in: bool) -> LoginReplyPacket {
        LoginReplyPacket { logged_in }
    }
    pub fn get_logged_in(&self) -> bool {
        self.logged_in
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::LoginReply as u8)
            .unwrap_or_default();
        wtr.write_u8(self.logged_in as u8).unwrap_or_default();
        wtr
    }
}
impl LoginReplyPacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> LoginReplyPacket {
        let logged_in: bool = rdr.read_u8().unwrap_or_default() != 0; // convert to bool
        LoginReplyPacket { logged_in }
    }
}
