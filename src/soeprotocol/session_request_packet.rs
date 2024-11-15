use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::lib_utils::str_from_u8_nul_utf8_checked;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRequestPacket {
    pub session_id: u32,
    pub crc_length: u32,
    pub udp_length: u32,
    protocol: String,
}
#[wasm_bindgen]
impl SessionRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, crc_length: u32, udp_length: u32, protocol: String) -> Self {
        Self {
            session_id,
            crc_length,
            udp_length,
            protocol,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_length(&self) -> u32 {
        self.crc_length
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
    pub fn get_protocol(&self) -> String {
        self.protocol.clone()
    }
}

impl SessionRequestPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> SessionRequestPacket {
        let session_id = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_length = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let udp_length = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let protocol_data_position = _rdr.position() as usize;
        let raw_data = _rdr.into_inner();
        let protocol =
            str_from_u8_nul_utf8_checked(&raw_data[protocol_data_position..]).to_string();
        SessionRequestPacket::new(crc_length, session_id, udp_length, protocol)
    }
}
