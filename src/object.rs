use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::Vector3f;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Diffuse,
    Reflective,
    ReflectiveAndRefractive,
}

pub const DEFAULT_ALBEDO: Vector3f = Vector3f(0.18, 0.18, 0.18);

pub const IOR_WATER: f64 = 1.3;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 1.8;

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub texture: Box<Texture>,
    pub albedo: Vector3f,
    pub material_type: MaterialType,
    pub ior: f64,
}

impl Object {
    pub fn new(
        name: &'static str,
        shape: Box<Shape>,
        texture: Box<Texture>,
        albedo: Color,
        material_type: MaterialType,
        ior: f64,
    ) -> Object {
        Object {
            name: name,
            shape: shape,
            texture: texture,
            albedo: albedo,
            material_type: material_type,
            ior: ior,
        }
    }
}
