use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::Vector3f;

#[derive(Debug)]
pub enum MaterialType {
    Diffuse,
    Reflective,
}

pub const DEFAULT_ALBEDO: Vector3f = Vector3f(0.18, 0.18, 0.18);

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub texture: Box<Texture>,
    pub albedo: Vector3f,
    pub material_type: MaterialType,
}

impl Object {
    pub fn new(
        name: &'static str,
        shape: Box<Shape>,
        texture: Box<Texture>,
        albedo: Color,
        material_type: MaterialType,
    ) -> Object {
        Object {
            name: name,
            shape: shape,
            texture: texture,
            albedo: albedo,
            material_type: material_type,
        }
    }
}
