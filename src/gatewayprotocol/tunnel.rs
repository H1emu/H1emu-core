use byteorder::WriteBytesExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::protocol::GatewayOpcode;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
// Internal
pub struct TunnelPacket {
    pub channel: u8,
    pub opcode: GatewayOpcode,
    tunnel_data: Vec<u8>,
}
#[wasm_bindgen]
impl TunnelPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(opcode: GatewayOpcode, channel: u8, tunnel_data: Vec<u8>) -> TunnelPacket {
        TunnelPacket {
            opcode,
            channel,
            tunnel_data,
        }
    }
    pub fn get_channel(&self) -> u8 {
        self.channel
    }
    pub fn get_tunnel_data(&self) -> Vec<u8> {
        self.tunnel_data.clone()
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut wtr: Vec<u8> = vec![];
        wtr.write_u8(self.opcode as u8).unwrap_or_default();
        wtr.append(&mut self.tunnel_data);
        wtr
    }
}
impl TunnelPacket {
    pub fn from(mut data: std::vec::Vec<u8>) -> TunnelPacket {
        let channel = data.remove(0) >> 5;
        let tunnel_data = data;
        // FIXME: opcode
        TunnelPacket {
            opcode: GatewayOpcode::TunnelDataServer,
            channel,
            tunnel_data,
        }
    }
}
