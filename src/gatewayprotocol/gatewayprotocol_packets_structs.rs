use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use std::io::Cursor;

use super::{
    channel_is_not_routable::ChannelIsNotRoutablePacket,
    channel_is_routable::ChannelIsRoutablePacket, force_disconnect::ForceDisconnectPacket,
    login_reply::LoginReplyPacket, login_request::LoginRequestPacket, logout::LogoutPacket,
    protocol::GatewayOpcode, tunnel::TunnelPacket,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GatewayPacket {
    LoginRequest(LoginRequestPacket),
    LoginReply(LoginReplyPacket),
    ChannelIsRoutable(ChannelIsRoutablePacket),
    ChannelIsNotRoutable(ChannelIsNotRoutablePacket),
    Tunnel(TunnelPacket),
    Unknown(UnknownGatewayPacket),
    Logout(LogoutPacket),
    ForceDisconnect(ForceDisconnectPacket),
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayPacketParsed {
    opcode: GatewayOpcode,
    packet: GatewayPacket,
}
#[wasm_bindgen]
impl GatewayPacketParsed {
    pub fn get_opcode(&self) -> GatewayOpcode {
        self.opcode
    }
    pub fn get_login_request_packet(&self) -> Option<LoginRequestPacket> {
        match &self.packet {
            GatewayPacket::LoginRequest(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_login_reply_packet(&self) -> Option<LoginReplyPacket> {
        match &self.packet {
            GatewayPacket::LoginReply(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_channel_is_routable_packet(&self) -> Option<ChannelIsRoutablePacket> {
        match &self.packet {
            GatewayPacket::ChannelIsRoutable(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_channel_is_not_routable_packet(&self) -> Option<ChannelIsNotRoutablePacket> {
        match &self.packet {
            GatewayPacket::ChannelIsNotRoutable(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_tunnel_packet(&self) -> Option<TunnelPacket> {
        match &self.packet {
            GatewayPacket::Tunnel(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_unknown_packet(&self) -> Option<UnknownGatewayPacket> {
        match &self.packet {
            GatewayPacket::Unknown(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_logout_packet(&self) -> Option<LogoutPacket> {
        match &self.packet {
            GatewayPacket::Logout(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_force_disconnect_packet(&self) -> Option<ForceDisconnectPacket> {
        match &self.packet {
            GatewayPacket::ForceDisconnect(packet) => Some(packet.clone()),
            _ => None,
        }
    }
}
impl GatewayPacketParsed {
    pub fn new(opcode: GatewayOpcode, packet: GatewayPacket) -> Self {
        Self { opcode, packet }
    }
    pub fn get_packet(&self) -> GatewayPacket {
        self.packet.clone()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnknownGatewayPacket {}
impl UnknownGatewayPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> UnknownGatewayPacket {
        UnknownGatewayPacket {}
    }
}
