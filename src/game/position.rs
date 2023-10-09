use serde::{Serialize, ser::{SerializeTuple, SerializeSeq}};

pub struct Point<T: Serialize>((T, T));

impl<T: Serialize> Point<T> {
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

impl<T: Serialize> Serialize for Point<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.0.0)?;
        state.serialize_element(&self.0.1)?;
        state.end()
    }
}

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

impl Serialize for Grid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in self.0.into_iter() {
            seq.serialize_element(row)?;
        }
        seq.end()
    }
}
