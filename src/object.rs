use material::Material;
use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::Vector3f;

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub albedo: f64,
    pub material: Material,
}

impl Object {
    pub fn new(name: &'static str, shape: Box<Shape>, albedo: f64, material: Material) -> Object {
        Object {
            name: name,
            shape: shape,
            albedo: albedo,
            material: material,
        }
    }
}
