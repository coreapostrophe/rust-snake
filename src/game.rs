use std::collections::HashMap;

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
        let mut hash_map = HashMap::new();
        hash_map.insert("width".to_string(), game_window.width());
        hash_map.insert("height".to_string(), game_window.height());

        serde_wasm_bindgen::to_value(&hash_map).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let game_world = self.world.as_ref().unwrap_or_else(|| {
            panic!("World is not found")
        });
        let mut hash_map: HashMap<String, f32> = HashMap::new();
        hash_map.insert("columns".to_string(), game_world.columns() as f32);
        hash_map.insert("rows".to_string(), game_world.rows() as f32);
        hash_map.insert("cell_size".to_string(), game_world.cell_size());
        
        serde_wasm_bindgen::to_value(&hash_map).unwrap()
    }

    pub fn generate_cells(&mut self) -> JsValue {
        let game_world = self.world.as_mut().unwrap_or_else(|| {
            panic!("World is not found")
        });
        let game_window = self.window.as_ref().unwrap_or_else(|| {
            panic!("Window is not found")
        });
        let cell_size = game_window.width() as f32 / game_world.columns() as f32;
        game_world.set_cell_size(cell_size as f32);

        let mut grid_vector = Vec::new();
        for row in 0..(game_world.rows()) {
            let mut column_vector = Vec::new();
            for column in 0..(game_world.columns()) {
                let x_coordinate = column as f32 * cell_size;
                let y_coordinate = row as f32 * cell_size;
                column_vector.push((y_coordinate, x_coordinate));
            }
            grid_vector.push(column_vector.into_boxed_slice());
        };

        serde_wasm_bindgen::to_value(&grid_vector.as_slice()).unwrap()
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
    pub fn set_world(mut self, columns: u32, rows: u32) -> Self {
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