use std::ops::{Add, AddAssign};

use serde::Serialize;

#[derive(Serialize)]
pub struct Point<T>((T, T));

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self((x, y))
    }
    pub fn x(&self) -> &T {
        &self.0.0
    }
    pub fn y(&self) -> &T {
        &self.0.1
    }
}

impl<T: AddAssign> Point<T> {
    pub fn translate_x(&mut self, units: T) {
        self.0.0 += units;
    }
    pub fn translate_y(&mut self, units: T) {
        self.0.1 += units;
    }
    pub fn translate(&mut self, x_units: T, y_units: T)  {
        self.translate_x(x_units);
        self.translate_y(y_units);
    }
}

impl<T: Clone + Serialize + Add<Output = T>> Point<T> {
    pub fn get_translate(&self, x_units: T, y_units: T) -> Self {
        Self((self.x().clone() + x_units, self.y().clone() + y_units))
    }
}

impl Point<f32> {
    pub fn distance(&self, point: &Self) -> f32 {
        let distance = f32::sqrt(f32::powi(self.x() - point.x(), 2) + f32::powi(self.y() - point.y(), 2));
        distance
    }
}

impl<T: PartialEq> Point<T> {
    pub fn is_equal(&self, point: &Self) -> bool {
        (self.x() == point.x()) & (self.y() == point.y())
    }
}

impl<T: Copy> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self((self.0.0, self.0.1))
    }
}

#[derive(Serialize)]
pub struct Grid(Box<[Box<[Point<f32>]>]>);

pub type PointClosure = Box<dyn Fn(u32, u32) -> Point<f32>>;

impl Grid {
    pub fn new(rows: u32, columns: u32, point_closure: Option<PointClosure>) -> Self {
        let mut grid_vector = Vec::new();
        for row in 0..rows {
            let mut column_vector = Vec::new();
            for column in 0..columns {
                match point_closure {
                    Some(ref closure) => column_vector.push(closure(row, column)),
                    None => column_vector.push(Point::new(row as f32, column as f32))
                }
            }
            grid_vector.push(column_vector.into_boxed_slice());
        }
        Self(grid_vector.into_boxed_slice())
    }
}
