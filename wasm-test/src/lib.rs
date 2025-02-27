use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn log_message() {
    web_sys::console::log_1(&"Hello from WebAssembly!".into());
}

#[wasm_bindgen]
pub fn process_file(file_bytes: &[u8]) {
    let file_size = file_bytes.len();
    web_sys::console::log_1(&format!("File size: {} bytes", file_size).into());
}
