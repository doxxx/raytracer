use std::f64;
use std::f64::consts::PI;

use color::Color;
use direction::Direction;
use point::Point;

#[derive(Debug, Clone)]
pub enum Light {
    Distant(DistantLight),
    Point(PointLight),
}

pub trait LightSource {
    /// Returns light direction, intensity/color and distance
    fn illuminate(&self, point: Point) -> (Direction, Color, f64);
}


#[derive(Debug, Clone)]
pub struct DistantLight {
    color: Color,
    intensity: f64,
    direction: Direction,
}

impl DistantLight {
    pub fn new(color: Color, intensity: f64, direction: Direction) -> DistantLight {
        DistantLight {
            color: color,
            intensity: intensity,
            direction: direction,
        }
    }
}

impl LightSource for DistantLight {
    fn illuminate(&self, _point: Point) -> (Direction, Color, f64) {
        (self.direction, self.color * self.intensity, f64::MAX)
    }
}


#[derive(Debug, Clone)]
pub struct PointLight {
    color: Color,
    intensity: f64,
    origin: Point,
}

impl PointLight {
    pub fn new(color: Color, intensity: f64, origin: Point) -> PointLight {
        PointLight {
            color: color,
            intensity: intensity,
            origin: origin,
        }
    }
}

impl LightSource for PointLight {
    fn illuminate(&self, point: Point) -> (Direction, Color, f64) {
        let mut dir = point - self.origin;
        let r2 = dir.length_squared();
        let distance = r2.sqrt();
        dir /= distance;
        (dir, self.color * self.intensity / (4.0 * PI * r2), distance)
    }
}
