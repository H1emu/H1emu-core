use byteorder::ReadBytesExt;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use super::{
    channel_is_not_routable::ChannelIsNotRoutablePacket,
    channel_is_routable::ChannelIsRoutablePacket,
    force_disconnect::ForceDisconnectPacket,
    gatewayprotocol_packets_structs::{GatewayPacket, GatewayPacketParsed, UnknownGatewayPacket},
    login_reply::LoginReplyPacket,
    login_request::LoginRequestPacket,
    logout::LogoutPacket,
    tunnel::TunnelPacket,
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GatewayOpcode {
    LoginRequest = 0x01,
    LoginReply = 0x02,
    Logout = 0x03,
    ForceDisconnect = 0x04,
    TunnelDataClient = 0x05,
    TunnelDataServer = 0x06,
    ChannelIsRoutable = 0x07,
    ChannelIsNotRoutable = 0x08,
    Unknown = 0x00,
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayChannels {
    Zone = 0,
    World = 1,
    UpdatePosition = 2,
    ShortCircuitZone = 3,
    Gateway = 4,
}

#[wasm_bindgen]
pub struct GatewayProtocol {}

#[wasm_bindgen]
impl GatewayProtocol {
    #[wasm_bindgen(constructor)]
    pub fn initialize() -> GatewayProtocol {
        GatewayProtocol {}
    }
    pub fn get_opcode(&mut self, opcode: u8) -> GatewayOpcode {
        match opcode {
            0x01 => GatewayOpcode::LoginRequest,
            0x02 => GatewayOpcode::LoginReply,
            0x03 => GatewayOpcode::Logout,
            0x04 => GatewayOpcode::ForceDisconnect,
            0x05 => GatewayOpcode::TunnelDataClient,
            0x06 => GatewayOpcode::TunnelDataServer,
            0x07 => GatewayOpcode::ChannelIsRoutable,
            0x08 => GatewayOpcode::ChannelIsNotRoutable,
            _ => GatewayOpcode::Unknown,
        }
    }
    pub fn parse(&mut self, data: Vec<u8>) -> GatewayPacketParsed {
        let mut rdr = Cursor::new(&data);
        if data.len() < 2 {
            return GatewayPacketParsed::new(
                GatewayOpcode::Unknown,
                GatewayPacket::Unknown(UnknownGatewayPacket {}),
            );
        }
        let full_opcode = rdr.read_u8().unwrap_or_default();
        let opcode = full_opcode & 0x1f;
        // let channel = full_opcode >> 5;

        let gateway_opcode = self.get_opcode(opcode);

        match gateway_opcode {
            GatewayOpcode::Unknown => GatewayPacketParsed::new(
                GatewayOpcode::Unknown,
                GatewayPacket::Unknown(UnknownGatewayPacket {}),
            ),
            GatewayOpcode::LoginRequest => GatewayPacketParsed::new(
                GatewayOpcode::LoginRequest,
                GatewayPacket::LoginRequest(LoginRequestPacket::from(rdr)),
            ),
            GatewayOpcode::LoginReply => GatewayPacketParsed::new(
                GatewayOpcode::LoginReply,
                GatewayPacket::LoginReply(LoginReplyPacket::from(rdr)),
            ),
            GatewayOpcode::Logout => GatewayPacketParsed::new(
                GatewayOpcode::Logout,
                GatewayPacket::Logout(LogoutPacket::from(rdr)),
            ),
            GatewayOpcode::ForceDisconnect => GatewayPacketParsed::new(
                GatewayOpcode::ForceDisconnect,
                GatewayPacket::ForceDisconnect(ForceDisconnectPacket::from(rdr)),
            ),
            GatewayOpcode::TunnelDataClient => GatewayPacketParsed::new(
                GatewayOpcode::TunnelDataClient,
                GatewayPacket::Tunnel(TunnelPacket::from(data)),
            ),
            GatewayOpcode::TunnelDataServer => GatewayPacketParsed::new(
                GatewayOpcode::TunnelDataServer,
                GatewayPacket::Tunnel(TunnelPacket::from(data)),
            ),
            GatewayOpcode::ChannelIsRoutable => GatewayPacketParsed::new(
                GatewayOpcode::ChannelIsRoutable,
                GatewayPacket::ChannelIsRoutable(ChannelIsRoutablePacket::from(rdr)),
            ),
            GatewayOpcode::ChannelIsNotRoutable => GatewayPacketParsed::new(
                GatewayOpcode::ChannelIsNotRoutable,
                GatewayPacket::ChannelIsNotRoutable(ChannelIsNotRoutablePacket::from(rdr)),
            ),
        }
    }
}
