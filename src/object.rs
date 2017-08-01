use material::Material;
use shapes::Shape;

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Shape,
    pub albedo: f64,
    pub material: Material,
}

impl Object {
    pub fn new(name: &'static str, shape: Shape, albedo: f64, material: Material) -> Object {
        Object {
            name: name,
            shape: shape,
            albedo: albedo,
            material: material,
        }
    }
}
