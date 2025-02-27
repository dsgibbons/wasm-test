use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn log_message() {
    web_sys::console::log_1(&"Hello from WebAssembly!".into());
}
