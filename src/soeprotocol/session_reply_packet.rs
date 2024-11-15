use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::soeprotocol::protocol::SoeOpcode;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionReplyPacket {
    pub opcode: u16,
    pub session_id: u32,
    pub crc_seed: u32,
    pub crc_length: u8,
    // TODO: use the EncryptionMethod enum
    pub encrypt_method: u16,
    pub udp_length: u32,
    pub bufferable: bool,
    pub length: u16,
}
#[wasm_bindgen]
impl SessionReplyPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        session_id: u32,
        crc_seed: u32,
        crc_length: u8,
        encrypt_method: u16,
        udp_length: u32,
    ) -> Self {
        let length = 15 as u16;
        Self {
            opcode: SoeOpcode::SessionReply as u16,
            session_id,
            crc_seed,
            crc_length,
            encrypt_method,
            udp_length,
            bufferable: false,
            length,
        }
    }
    pub fn get_session_id(&self) -> u32 {
        self.session_id
    }
    pub fn get_crc_seed(&self) -> u32 {
        self.crc_seed
    }
    pub fn get_crc_length(&self) -> u8 {
        self.crc_length
    }
    pub fn get_encrypt_method(&self) -> u16 {
        self.encrypt_method
    }
    pub fn get_udp_length(&self) -> u32 {
        self.udp_length
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u16::<BigEndian>(SoeOpcode::SessionReply as u16)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.session_id)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.crc_seed)
            .unwrap_or_default();
        wtr.write_u8(self.crc_length).unwrap_or_default();
        wtr.write_u16::<BigEndian>(self.encrypt_method)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(self.udp_length)
            .unwrap_or_default();
        wtr.write_u32::<BigEndian>(3).unwrap_or_default();
        wtr
    }
}

impl SessionReplyPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> SessionReplyPacket {
        let session_id = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_seed = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let crc_length = _rdr.read_u8().unwrap_or_default();
        let encrypt_method = _rdr.read_u16::<BigEndian>().unwrap_or_default();
        let udp_length = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        SessionReplyPacket::new(session_id, crc_seed, crc_length, encrypt_method, udp_length)
    }
}
