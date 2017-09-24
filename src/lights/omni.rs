use std::f64;
use std::f64::consts::PI;

use color::Color;
use direction::Direction;
use point::Point;

use lights::Light;

pub struct Omni {
    pub color: Color,
    pub intensity: f64,
    pub origin: Point,
}

impl Light for Omni {
    fn illuminate(&self, point: Point) -> (Direction, Color, f64) {
        let mut dir = point - self.origin;
        let r2 = dir.length_squared();
        let distance = r2.sqrt();
        dir /= distance;
        (dir, self.color * self.intensity / (4.0 * PI * r2), distance)
    }
}
