use matrix::Matrix44f;
use shader::Shader;
use shapes::Shape;
use system::{Intersection, Ray, Intersectable, Transformable};

pub const DEFAULT_ALBEDO: f64 = 0.18;

type ShaderApplication = (f64,Shader);

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub shape: Shape,
    pub shaders: Vec<ShaderApplication>,
}

impl Object {
    pub fn new(name: &str, shape: Shape, shaders: Vec<ShaderApplication>) -> Object {
        Object {
            name: String::from(name),
            shape: shape,
            shaders: shaders,
        }
    }
}

impl Transformable for Object {
    fn transform(&self, m: Matrix44f) -> Self {
        Object {
            name: self.name.clone(),
            shape: self.shape.transform(m),
            shaders: self.shaders.clone(),
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray)
    }
}
