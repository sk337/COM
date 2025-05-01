use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn disassemble(bytes: Uint8Array) -> String {
    let bytes = bytes.to_vec();
    let disassembler = disassembler::disassemble::Disassembler::new(bytes);

    return disassembler.to_string();
}
