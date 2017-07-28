use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::Vector3f;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Diffuse(Color),
    Reflective,
    ReflectiveAndRefractive(f64),
}

pub const DEFAULT_ALBEDO: f64 = 0.18;

pub const IOR_WATER: f64 = 1.3;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 1.8;

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub albedo: f64,
    pub material: Material,
}

impl Object {
    pub fn new(
        name: &'static str,
        shape: Box<Shape>,
        albedo: f64,
        material: Material,
    ) -> Object {
        Object {
            name: name,
            shape: shape,
            albedo: albedo,
            material: material,
        }
    }
}
