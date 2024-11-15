use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::lib_utils::read_prefixed_string_le;

use super::protocol::GatewayOpcode;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequestPacket {
    pub character_id: u64,
    ticket: String,
    client_protocol: String,
    client_build: String,
}
#[wasm_bindgen]
impl LoginRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        character_id: u64,
        ticket: String,
        client_protocol: String,
        client_build: String,
    ) -> LoginRequestPacket {
        LoginRequestPacket {
            character_id,
            ticket,
            client_protocol,
            client_build,
        }
    }
    pub fn get_character_id(&self) -> u64 {
        self.character_id
    }
    pub fn get_ticket(&self) -> String {
        self.ticket.clone()
    }
    pub fn get_client_protocol(&self) -> String {
        self.client_protocol.clone()
    }
    pub fn get_client_build(&self) -> String {
        self.client_build.clone()
    }
    pub fn build(&self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(GatewayOpcode::LoginRequest as u8)
            .unwrap_or_default();
        wtr.write_u64::<LittleEndian>(self.character_id)
            .unwrap_or_default();
        let ticket = self.get_ticket();
        wtr.write_u32::<LittleEndian>(ticket.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut ticket.as_bytes().to_vec());
        let client_protocol = self.get_client_protocol();
        wtr.write_u32::<LittleEndian>(client_protocol.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut client_protocol.as_bytes().to_vec());
        let client_build = self.get_client_build();
        wtr.write_u32::<LittleEndian>(client_build.len() as u32)
            .unwrap_or_default();
        wtr.append(&mut client_build.as_bytes().to_vec());
        wtr
    }
}
impl LoginRequestPacket {
    pub fn from(mut rdr: Cursor<&std::vec::Vec<u8>>) -> LoginRequestPacket {
        let character_id = rdr.read_u64::<LittleEndian>().unwrap_or_default();
        let raw_data = rdr.clone().into_inner();
        let ticket_data_pos = rdr.position();
        let ticket_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let ticket = read_prefixed_string_le(raw_data, ticket_data_pos as usize, ticket_data_len);
        rdr.set_position(ticket_data_pos + ticket_data_len as u64 + 4);
        let client_protocol_data_pos = rdr.position();
        let client_protocol_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let client_protocol = read_prefixed_string_le(
            raw_data,
            client_protocol_data_pos as usize,
            client_protocol_data_len,
        );
        rdr.set_position(client_protocol_data_pos + client_protocol_data_len as u64 + 4);
        let client_build_data_pos = rdr.position();
        let client_build_data_len = rdr.read_u32::<LittleEndian>().unwrap_or_default();
        let client_build = read_prefixed_string_le(
            raw_data,
            client_build_data_pos as usize,
            client_build_data_len,
        );
        LoginRequestPacket {
            character_id,
            ticket,
            client_protocol,
            client_build,
        }
    }
}
