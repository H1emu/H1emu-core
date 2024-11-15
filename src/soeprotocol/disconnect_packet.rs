use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub fn disconnect_reason_to_string(reason_id: u16) -> String {
    match reason_id {
        0 => "DisconnectReasonIcmpError".to_string(),
        1 => "DisconnectReasonTimeout".to_string(),
        2 => "DisconnectReasonNone".to_string(),
        3 => "DisconnectReasonOtherSideTerminated".to_string(),
        4 => "DisconnectReasonManagerDeleted".to_string(),
        5 => "DisconnectReasonConnectFail".to_string(),
        6 => "DisconnectReasonApplication".to_string(),
        7 => "DisconnectReasonUnreachableConnection".to_string(),
        8 => "DisconnectReasonUnacknowledgedTimeout".to_string(),
        9 => "DisconnectReasonNewConnectionAttempt".to_string(),
        10 => "DisconnectReasonConnectionRefused".to_string(),
        11 => "DisconnectReasonConnectErro".to_string(),
        12 => "DisconnectReasonConnectingToSelf".to_string(),
        13 => "DisconnectReasonReliableOverflow".to_string(),
        14 => "DisconnectReasonApplicationReleased".to_string(),
        15 => "DisconnectReasonCorruptPacket".to_string(),
        16 => "DisconnectReasonProtocolMismatch".to_string(),
        _ => "unknown".to_string(),
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisconnectPacket {
    pub session_id: u32,
    reason: String,
}
#[wasm_bindgen]
impl DisconnectPacket {
    #[wasm_bindgen(constructor)]
    pub fn new(session_id: u32, reason: String) -> Self {
        Self { session_id, reason }
    }
    pub fn get_reason(&self) -> String {
        self.reason.clone()
    }
}

impl DisconnectPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> DisconnectPacket {
        let session_id = _rdr.read_u32::<BigEndian>().unwrap_or_default();
        let reason = disconnect_reason_to_string(_rdr.read_u16::<BigEndian>().unwrap_or_default());
        DisconnectPacket::new(session_id, reason)
    }
}
