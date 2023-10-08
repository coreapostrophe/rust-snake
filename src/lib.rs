use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let formatter_str = format!("Hello, {}!", name);
    formatter_str
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
