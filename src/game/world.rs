
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct World {
    columns: u32,
    rows: u32,
    cell_size: f32
}

impl World {
    pub fn new(columns: u32, rows: u32) -> Self {
        Self {
            columns,
            rows,
            cell_size: 1.0
        }
    }
    pub fn columns(&self) -> u32 {
        self.columns
    }
    pub fn rows(&self) -> u32 {
        self.rows
    }
    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
    pub fn set_cell_size(&mut self, size_pixels: f32) {
        self.cell_size = size_pixels;
    }
}