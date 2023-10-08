use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct World {
    columns: usize,
    rows: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
        }
    }
    pub fn columns(&self) -> usize {
        self.columns
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
}