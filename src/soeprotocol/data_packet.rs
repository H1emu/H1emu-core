use std::io::Cursor;

use super::soeprotocol_functions::{get_data_end, write_packet_data};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct DataPacket {
    data: Vec<u8>,
    pub opcode: u16,
    pub sequence: u16,
    pub bufferable: bool,
    pub length: u16,
}
#[wasm_bindgen]
impl DataPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>, sequence: u16, opcode: u16) -> Self {
        let length = 4 + data.len() as u16;
        Self {
            data,
            sequence,
            opcode,
            bufferable: true,
            length,
        }
    }
    pub fn get_sequence(&self) -> u16 {
        self.sequence
    }

    pub fn get_data(&mut self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u16::<BigEndian>(self.opcode).unwrap_or_default();
        write_packet_data(&mut wtr, self);
        wtr
    }
}
impl DataPacket {
    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>, opcode: u16, use_crc: bool) -> DataPacket {
        let sequence = _rdr.read_u16::<BigEndian>().unwrap_or_default();

        let data_end: u64 = get_data_end(&_rdr, use_crc);
        let vec = _rdr.get_ref().to_vec();
        let data = &vec[4..data_end as usize];
        DataPacket::new(data.to_vec(), sequence, opcode)
    }
}
