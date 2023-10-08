use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl World {
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
}

#[wasm_bindgen]
pub struct WorldBuilder {
    width: usize,
    height: usize
}

#[wasm_bindgen]
impl WorldBuilder {
    pub fn new(width_units: Option<usize>, height_units: Option<usize>) -> Self {
        Self {
            width: width_units.unwrap_or(0),
            height: height_units.unwrap_or(0)
        }
    }
    
    pub fn set_width(mut self, width_units: usize) -> Self {
        self.width = width_units;
        self
    }

    pub fn set_height(mut self, height_units: usize) -> Self {
        self.height = height_units;
        self
    }
    
    pub fn build(self) -> World {
        World {
            width: self.width,
            height: self.height
        }
    }
}