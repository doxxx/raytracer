use material::Material;
use matrix::Matrix44f;
use shapes::Shape;
use system::{Intersection, Ray, Intersectable, Transformable};

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub shape: Shape,
    pub albedo: f64,
    pub material: Material,
}

impl Object {
    pub fn new(name: &str, shape: Shape, albedo: f64, material: Material) -> Object {
        Object {
            name: String::from(name),
            shape: shape,
            albedo: albedo,
            material: material,
        }
    }
}

impl Transformable for Object {
    fn transform(&self, m: Matrix44f) -> Self {
        Object {
            name: self.name.clone(),
            shape: self.shape.transform(m),
            albedo: self.albedo,
            material: self.material,
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }
}
