use std::io::Cursor;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogoutPacket {}
impl LogoutPacket {
    pub fn from(mut _rdr: Cursor<&std::vec::Vec<u8>>) -> LogoutPacket {
        LogoutPacket {}
    }
}
