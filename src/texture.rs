use std::f64;
use std::fmt;

use image::{DynamicImage, GenericImage, Pixel};

use crate::color::Color;
use crate::vector::Vector2f;

pub trait ColorSource {
    fn color_at_uv(&self, uv: Vector2f) -> Color;
}

#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Pattern(Pattern),
    Image(DynamicImage, f64),
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Texture::Solid(ref c) => f.debug_tuple("Texture::Solid").field(c).finish(),
            &Texture::Pattern(ref p) => f.debug_tuple("Texture::Pattern").field(p).finish(),
            &Texture::Image(ref i, s) => f
                .debug_struct("Texture::Image")
                .field("width", &i.width())
                .field("height", &i.height())
                .field("scale", &s)
                .finish(),
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Texture) -> bool {
        if let &Texture::Solid(ref c1) = self {
            if let &Texture::Solid(ref c2) = other {
                return c1 == c2;
            }
        } else if let &Texture::Pattern(ref p1) = self {
            if let &Texture::Pattern(ref p2) = other {
                return p1 == p2;
            }
        } else if let &Texture::Image(ref i1, ref s1) = self {
            if let &Texture::Image(ref i2, ref s2) = other {
                return i1.pixels().eq(i2.pixels()) && s1 == s2;
            }
        }
        return false;
    }
}

impl ColorSource for Texture {
    fn color_at_uv(&self, uv: Vector2f) -> Color {
        match self {
            &Texture::Solid(color) => color,
            &Texture::Pattern(ref pattern) => pattern.color_at_uv(uv),
            &Texture::Image(ref image, scale) => {
                let max_x = (image.width() - 1) as f64;
                let max_y = (image.height() - 1) as f64;
                let x = ((uv.0 * scale * max_x) as u32) % image.width();
                let y = ((uv.1 * scale * max_y) as u32) % image.height();
                let p = image.get_pixel(x, y);
                let c = p.channels();
                Color::new((c[0] as f64) / 255.0, (c[1] as f64) / 255.0, (c[2] as f64) / 255.0)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

fn mix(a: Color, b: Color, v: f64) -> Color {
    a * (1.0 - v) + b * v
}
