use std::f64;
use std::f64::consts::PI;

use color::Color;
use direction::Direction;
use point::Point;

#[derive(Debug, Clone)]
pub enum Light {
    Distant {
        color: Color,
        intensity: f64,
        direction: Direction,
    },
    Point {
        color: Color,
        intensity: f64,
        origin: Point,
    },
}

impl Light {
    pub fn illuminate(&self, point: Point) -> (Direction, Color, f64) {
        match self {
            &Light::Distant { color: c, intensity: i, direction: d } =>
                (d, c * i, f64::MAX),
            &Light::Point { color: c, intensity: i, origin: o } => {
                let mut dir = point - o;
                let r2 = dir.length_squared();
                let distance = r2.sqrt();
                dir /= distance;
                (dir, c * i / (4.0 * PI * r2), distance)
            }
        }
    }
}
