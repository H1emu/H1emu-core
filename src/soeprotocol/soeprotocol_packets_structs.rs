use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::soeprotocol::protocol::SoeOpcode;

use super::{
    ack_packet::AckPacket,
    data_packet::DataPacket,
    disconnect_packet::DisconnectPacket,
    fatal_error_packet::FatalErrorPacket,
    multi_packets::{GroupedPackets, MultiPackablePacket},
    net_status_reply_packet::NetStatusReplyPacket,
    net_status_request_packet::NetStatusRequestPacket,
    ping_packet::PingPacket,
    session_reply_packet::SessionReplyPacket,
    session_request_packet::SessionRequestPacket,
    unknown_packet::UnknownPacket,
};

// #[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SoePacket {
    SessionRequestPacket(SessionRequestPacket),
    SessionReplyPacket(SessionReplyPacket),
    NetStatusReplyPacket(NetStatusReplyPacket),
    MultiPackablePacket(MultiPackablePacket),
    DataPacket(DataPacket),
    AckPacket(AckPacket),
    NetStatusRequestPacket(NetStatusRequestPacket),
    GroupedPackets(GroupedPackets),
    PingPacket(PingPacket),
    FatalErrorPacket(FatalErrorPacket),
    DisconnectPacket(DisconnectPacket),
    UnknownPacket(UnknownPacket),
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoePacketParsed {
    pub opcode: SoeOpcode,
    packet: SoePacket,
}
#[wasm_bindgen]
impl SoePacketParsed {
    pub fn get_opcode(&self) -> SoeOpcode {
        self.opcode
    }
    pub fn get_session_request_packet(&self) -> Option<SessionRequestPacket> {
        match &self.packet {
            SoePacket::SessionRequestPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    // TODO: remove all clones like that
    pub fn get_session_reply_packet(self) -> Option<SessionReplyPacket> {
        match self.packet {
            SoePacket::SessionReplyPacket(packet) => Some(packet),
            _ => None,
        }
    }
    pub fn get_net_status_reply_packet(&self) -> Option<NetStatusReplyPacket> {
        match &self.packet {
            SoePacket::NetStatusReplyPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_data_packet(&self) -> Option<DataPacket> {
        match &self.packet {
            SoePacket::DataPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_ack_packet(&self) -> Option<AckPacket> {
        match &self.packet {
            SoePacket::AckPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_net_status_request_packet(&self) -> Option<NetStatusRequestPacket> {
        match &self.packet {
            SoePacket::NetStatusRequestPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_grouped_packets(&self) -> Option<GroupedPackets> {
        match &self.packet {
            SoePacket::GroupedPackets(packet) => Some(packet.clone()),
            _ => None,
        }
    }
    pub fn get_unknown_packet(&self) -> Option<UnknownPacket> {
        match &self.packet {
            SoePacket::UnknownPacket(packet) => Some(packet.clone()),
            _ => None,
        }
    }
}
impl SoePacketParsed {
    pub fn new(opcode: SoeOpcode, packet: SoePacket) -> Self {
        Self { opcode, packet }
    }
    pub fn get_packet(&self) -> SoePacket {
        self.packet.clone()
    }
}
