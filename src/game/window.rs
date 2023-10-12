use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Window{
    width: u32,
    height: u32
}

#[wasm_bindgen]
impl Window {
    #[wasm_bindgen(constructor)]
    pub fn new(width_pixels: u32, height_pixels: u32) -> Self {
        Self {
            width: width_pixels,
            height: height_pixels
        }
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}