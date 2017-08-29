use std::f64;
use std::fmt::Debug;

use color::Color;
use vector::Vector2f;

fn mix(a: Color, b: Color, v: f64) -> Color {
    a * (1.0 - v) + b * v
}

pub trait Texture: Debug {
    fn color(&self, uv: Vector2f) -> Color;
}

#[derive(Debug, Copy, Clone)]
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

impl Texture for Checkerboard {
    fn color(&self, uv: Vector2f) -> Color {
        let scaled_x_frac = (uv.0 * self.scale).fract();
        let scaled_y_frac = (uv.1 * self.scale).fract();
        let x_pattern = (scaled_x_frac.abs() > 0.5) ^ (scaled_x_frac < 0.0);
        let y_pattern = (scaled_y_frac.abs() > 0.5) ^ (scaled_y_frac < 0.0);
        let pattern = if x_pattern ^ y_pattern { 1.0 } else { 0.0 };
        mix(self.color1, self.color2, pattern)
    }
}
