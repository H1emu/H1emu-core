use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FatalErrorPacket {}
#[wasm_bindgen]
impl FatalErrorPacket {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}
