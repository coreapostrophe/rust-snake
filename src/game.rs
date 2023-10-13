use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use self::position::{Point, Grid};
use self::snake::Snake;
use self::{window::Window, world::World};

pub mod position;
pub mod window;
pub mod world;
pub mod snake;

#[wasm_bindgen]
pub struct SnakeEngine {
    window: Option<Window>,
    world: Option<World>,
    snake: Option<Snake>
}

#[wasm_bindgen]
impl SnakeEngine {
    pub fn window(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.window).unwrap()
    }

    pub fn snake(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.snake).unwrap()
    }

    pub fn world(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.world).unwrap()
    }

    pub fn generate_snake(&mut self) -> JsValue {
        let game_world = self.world.as_ref().unwrap_or_else(|| {
            panic!("World is not found")
        });
        let columns = game_world.columns();
        let rows = game_world.rows();
        let cell_size = game_world.cell_size();

        let snake_head: Point<f32> = {
            let x_coordinate = (columns as f32 * 0.25).floor();
            let y_coordinate = (rows as f32 * 0.5).floor();
            Point::new(x_coordinate, y_coordinate)
        };
        let snake_segment = snake_head.get_translate(-1.0, 0.0);

        let snake = Snake::new(vec![
            snake_head,
            snake_segment
        ]);
        let snake_in_cells = self.transform_snake_to_cells(&snake, &cell_size);
        self.snake = Some(snake);

        serde_wasm_bindgen::to_value(&snake_in_cells).unwrap()
    }

    pub fn generate_cells(&mut self) -> JsValue {
        let game_world = self.world.as_mut().unwrap_or_else(|| {
            panic!("World is not found")
        });
        let game_window = self.window.as_ref().unwrap_or_else(|| {
            panic!("Window is not found")
        });
        let cell_size: f32 = game_window.width() as f32 / game_world.columns() as f32;
        game_world.set_cell_size(cell_size);

        let grid_vector = Grid::new(
            game_world.rows(),
            game_world.columns(),
            Some(Box::new(move |row, column| {
                let x_coordinate = column as f32 * cell_size;
                let y_coordinate = row as f32 * cell_size;
                Point::new(y_coordinate, x_coordinate)
            }))
        );

        serde_wasm_bindgen::to_value(&grid_vector).unwrap()
    }
}

impl SnakeEngine {
    pub fn transform_snake_to_cells(&self, snake: &Snake, cell_size: &f32) -> Vec<Point<f32>> {
        let snake_body = snake.body();
        let mut new_snake_body = Vec::new();
        for segment in snake_body.iter() {
            new_snake_body.push(Point::new(segment.x() * cell_size, segment.y() * cell_size));
        }
        new_snake_body
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
            world: self.world,
            snake: None
        }
    }
}