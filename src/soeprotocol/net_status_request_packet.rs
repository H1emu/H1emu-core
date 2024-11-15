use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetStatusRequestPacket {
    pub client_tick_count: u16,
    pub last_client_update: u32,
    pub average_update: u32,
    pub shortest_update: u32,
    pub longest_update: u32,
    pub last_server_update: u32,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub unknown_field: u16,
}
#[wasm_bindgen]
impl NetStatusRequestPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(
        client_tick_count: u16,
        last_client_update: u32,
        average_update: u32,
        shortest_update: u32,
        longest_update: u32,
        last_server_update: u32,
        packets_sent: u64,
        packets_received: u64,
        unknown_field: u16,
    ) -> Self {
        Self {
            client_tick_count,
            last_client_update,
            average_update,
            shortest_update,
            longest_update,
            last_server_update,
            packets_sent,
            packets_received,
            unknown_field,
        }
    }
    pub fn get_client_tick_count(&self) -> u16 {
        self.client_tick_count
    }
    pub fn get_last_client_update(&self) -> u32 {
        self.last_client_update
    }
    pub fn get_average_update(&self) -> u32 {
        self.average_update
    }
    pub fn get_shortest_update(&self) -> u32 {
        self.shortest_update
    }
    pub fn get_longest_update(&self) -> u32 {
        self.longest_update
    }
    pub fn get_last_server_update(&self) -> u32 {
        self.last_server_update
    }
    pub fn get_packets_sent(&self) -> u64 {
        self.packets_sent
    }
    pub fn get_packets_received(&self) -> u64 {
        self.packets_received
    }
    pub fn get_unknown_field(&self) -> u16 {
        self.unknown_field
    }
}
impl NetStatusRequestPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> NetStatusRequestPacket {
        let client_tick_count = _rdr.read_u16::<BigEndian>().unwrap_or_default();
        let last_client_update = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let average_update = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let shortest_update = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let longest_update = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let last_server_update = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let packets_sent = _rdr.read_u64::<BigEndian>().unwrap_or_default();
        let packets_received = _rdr.read_u64::<BigEndian>().unwrap_or_default();
        let unknown_field = _rdr.read_u16::<BigEndian>().unwrap_or_default();
        NetStatusRequestPacket {
            client_tick_count,
            last_client_update,
            average_update,
            shortest_update,
            longest_update,
            last_server_update,
            packets_sent,
            packets_received,
            unknown_field,
        }
    }
}
