use std::{collections::{HashMap, hash_map}, fmt::Result};

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use self::{window::Window, world::World};

pub mod window;
pub mod world;

#[wasm_bindgen]
pub struct SnakeEngine {
    window: Option<Window>,
    world: Option<World>
}

#[wasm_bindgen]
impl SnakeEngine {
    pub fn window(&self) -> JsValue {
        let game_window = self.window.as_ref().unwrap_or_else(|| {
            panic!("Window is not found")
        });
        let window_width = game_window.width();
        let window_height = game_window.height();
        let mut hash_map = HashMap::new();
        hash_map.insert("width".to_string(), window_width);
        hash_map.insert("height".to_string(), window_height);
        
        serde_wasm_bindgen::to_value(&hash_map).unwrap()
    }
    
    pub fn world(&self) -> JsValue {
        let game_world = self.world.as_ref().unwrap_or_else(|| {
            panic!("World is not found")
        });
        let game_columns = game_world.columns();
        let game_rows = game_world.rows();
        let mut hash_map = HashMap::new();
        hash_map.insert("columns".to_string(), game_columns);
        hash_map.insert("rows".to_string(), game_rows);
        
        serde_wasm_bindgen::to_value(&hash_map).unwrap()
    }
}

#[wasm_bindgen]
pub struct SnakeEngineBuilder {
    window: Option<Window>,
    world: Option<World>
}

#[wasm_bindgen]
impl SnakeEngineBuilder {
    pub fn new() -> Self {
        Self {
            window: None,
            world: None
        }
    }
    
    pub fn set_window(mut self, width_pixels: u32, height_pixels: u32) -> Self {
        self.window = Some(Window::new(width_pixels, height_pixels));
        self
    }
    pub fn set_world(mut self, columns: usize, rows: usize) -> Self {
        self.world = Some(World::new(columns, rows));
        self
    }
    pub fn build(self) -> SnakeEngine {
        SnakeEngine {
            window: self.window,
            world: self.world
        }
    }
}