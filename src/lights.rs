use std::f64;
use std::f64::consts::PI;
use std::fmt::Debug;

use system::Color;
use vector::Vector3f;


pub trait Light: Debug {
    /// Returns light direction, intensity/color and distance
    fn illuminate(&self, point: Vector3f) -> (Vector3f, Color, f64);
}


#[derive(Debug)]
pub struct DistantLight {
    color: Color,
    intensity: f64,
    direction: Vector3f,
}

impl DistantLight {
    pub fn new(color: Color, intensity: f64, direction: Vector3f) -> DistantLight {
        DistantLight {
            color: color,
            intensity: intensity,
            direction: direction,
        }
    }
}

impl Light for DistantLight {
    fn illuminate(&self, point: Vector3f) -> (Vector3f, Color, f64) {
        (self.direction, self.color * self.intensity, f64::MAX)
    }
}


#[derive(Debug)]
pub struct PointLight {
    color: Color,
    intensity: f64,
    origin: Vector3f,
}

impl PointLight {
    pub fn new(color: Color, intensity: f64, origin: Vector3f) -> PointLight {
        PointLight {
            color: color,
            intensity: intensity,
            origin: origin,
        }
    }
}

impl Light for PointLight {
    fn illuminate(&self, point: Vector3f) -> (Vector3f, Color, f64) {
        let mut dir = point - self.origin;
        let r2 = dir.length_squared();
        let distance = r2.sqrt();
        dir /= distance;
        (dir, self.color * self.intensity / (4.0 * PI * r2), distance)
    }
}
