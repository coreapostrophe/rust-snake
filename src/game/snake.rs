use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

use super::position::Point;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Snake {
    body: Vec<Point<f32>>
}

impl Snake {
    pub fn new(area_points: Vec<Point<f32>>) -> Self {
        Self {
            body: area_points,
        }
    }
    pub fn body(&self) -> &Vec<Point<f32>> {
        &self.body
    }
    pub fn mut_body(&mut self) -> &mut Vec<Point<f32>> {
        &mut self.body
    }
}