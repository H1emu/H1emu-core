use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::soeprotocol_packets_structs::{SoePacket, SoePacketParsed};
use crate::soeprotocol::protocol::SoeOpcode::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiPackablePacket {
    // should contain all possible field for a multiPackable packet
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub sequence: u16,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupedPackets {
    sub_packets: Vec<SoePacket>,
}
#[wasm_bindgen]
impl GroupedPackets {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sub_packets: Vec::new(),
        }
    }
    pub fn get_packets(&self) -> Vec<SoePacketParsed> {
        let mut vec: Vec<SoePacketParsed> = vec![];
        for soe_packet in self.sub_packets.clone() {
            let opcode = match &soe_packet {
                session_request_packet => SessionReply,
                session_reply_packet => SessionReply,
                data_packet => Data,
                ack_packet => Ack,
                disconnect_packet => Disconnect,
                ping_packet => Ping,
                unknown_packet => Unknown,
                _ => Unknown,
            };
            vec.push(SoePacketParsed::new(opcode, soe_packet));
        }
        vec
    }
}
impl GroupedPackets {
    pub fn add_packet(&mut self, packet: SoePacket) {
        self.sub_packets.push(packet);
    }
}
