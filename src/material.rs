use std::f64;
use vector::Vector2f;

use vector::Vector3f;

pub type Color = Vector3f;

fn mix(a: Color, b: Color, v: f64) -> Color {
    a * (1.0 - v) + b * v
}

pub trait Material {
    fn color(&self, texture_coords: Vector2f) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct Flat {
    color: Color,
}

impl Flat {
    pub fn new(color: Color) -> Flat {
        Flat { 
            color: color
        }
    }
}

impl Material for Flat {
    fn color(&self, texture_coords: Vector2f) -> Color {
        return self.color;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Checkerboard {
    color1: Color,
    color2: Color,
    scale: f64,
}

impl Checkerboard {
    pub fn new(color1: Color, color2: Color, scale: f64) -> Checkerboard {
        Checkerboard {
            color1: color1,
            color2: color2,
            scale: scale,
        }
    }
}

impl Material for Checkerboard {
    fn color(&self, texture_coords: Vector2f) -> Color {
        let x_pattern = (texture_coords.0 * self.scale).fract() > 0.5;
        let y_pattern = (texture_coords.1 * self.scale).fract() > 0.5;
        let pattern = if x_pattern ^ y_pattern { 1.0 } else { 0.0 };
        mix(self.color1, self.color2, pattern)
    }
}
