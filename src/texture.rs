use std::f64;

use image::{Pixel,RgbImage};

use color::Color;
use vector::Vector2f;

pub trait ColorSource {
    fn color_at_uv(&self, uv: Vector2f) -> Color;
}

#[derive(Debug, Clone)]
pub enum Texture {
    Solid(Color),
    Pattern(Pattern),
    Image(RgbImage),
}

impl ColorSource for Texture {
    fn color_at_uv(&self, uv: Vector2f) -> Color {
        match self {
            &Texture::Solid(c) => c,
            &Texture::Pattern(ref p) => p.color_at_uv(uv),
            &Texture::Image(ref i) => i.color_at_uv(uv),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Checkerboard(Color, Color, f64),
}

impl ColorSource for Pattern {
    fn color_at_uv(&self, uv: Vector2f) -> Color {
        match self {
            &Pattern::Checkerboard(color1, color2, scale) => {
                let scaled_x_frac = (uv.0 * scale).fract();
                let scaled_y_frac = (uv.1 * scale).fract();
                let x_pattern = (scaled_x_frac.abs() > 0.5) ^ (scaled_x_frac < 0.0);
                let y_pattern = (scaled_y_frac.abs() > 0.5) ^ (scaled_y_frac < 0.0);
                let pattern = if x_pattern ^ y_pattern { 1.0 } else { 0.0 };
                mix(color1, color2, pattern)
            }
        }
    }
}

impl ColorSource for RgbImage {
    fn color_at_uv(&self, uv: Vector2f) -> Color {
        let x = (uv.0 * (self.width() as f64)) as u32;
        let y = (uv.1 * (self.height() as f64)) as u32;
        let p = self.get_pixel(x, y);
        let c = p.channels();
        Color::new((c[0] as f64) / 255.0, (c[1] as f64) / 255.0, (c[2] as f64) / 255.0)
    }
}

fn mix(a: Color, b: Color, v: f64) -> Color {
    a * (1.0 - v) + b * v
}
