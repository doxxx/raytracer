use material::Material;
use matrix::Matrix44f;
use shapes::{Shape,Transformable};

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Debug, Clone)]
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

    pub fn transform(self, m: Matrix44f) -> Self {
        Object {
            name: self.name,
            shape: match self.shape {
                Shape::Plane(s) => Shape::Plane(s.transform(m)),
                Shape::Triangle(s) => Shape::Triangle(s.transform(m)),
                Shape::Sphere(s) => Shape::Sphere(s.transform(m)),
                Shape::Mesh(s) => Shape::Mesh(s.transform(m)),
                Shape::Composite(s) => Shape::Composite(s.transform(m)),
            },
            albedo: self.albedo,
            material: self.material,
        }
    }
}
