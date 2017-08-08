use color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Diffuse(Color),
    Reflective,
    ReflectiveAndRefractive(f64),
}

pub const IOR_WATER: f64 = 1.3;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 1.8;
