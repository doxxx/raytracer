use std::f64;

use color::Color;
use direction::Direction;
use point::Point;

use lights::Light;

pub struct Distant {
    pub color: Color,
    pub intensity: f64,
    pub direction: Direction,
}

impl Light for Distant {
    fn illuminate(&self, point: Point) -> (Direction, Color, f64) {
        (self.direction, self.color * self.intensity, f64::MAX)
    }
}
