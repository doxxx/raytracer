use std::f64;

use color::Color;
use direction::Direction;
use point::Point;

pub mod distant;
pub mod omni;

pub trait Light: Send + Sync {
    fn illuminate(&self, point: Point) -> (Direction, Color, f64);
}
