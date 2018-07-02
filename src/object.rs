use materials::Material;
use matrix::Matrix44f;
use shapes::Shape;
use system::{Intersection, Ray, Intersectable, Transformable};

#[derive(Clone)]
pub struct Transformation {
    pub object_to_world: Matrix44f,
    pub world_to_object: Matrix44f,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            object_to_world: Matrix44f::identity(),
            world_to_object: Matrix44f::identity(),
        }
    }
}

impl Transformable for Transformation {
    fn transform(&mut self, m: Matrix44f) {
        self.object_to_world = self.object_to_world * m;
        self.world_to_object = self.object_to_world.inverse();
    }
}

pub struct Object {
    pub name: String,
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(name: &str, shape: Box<dyn Shape>, material: Box<dyn Material>) -> Object {
        Object {
            name: String::from(name),
            shape,
            material,
        }
    }
}

impl Transformable for Object {
    fn transform(&mut self, m: Matrix44f) {
        self.shape.transform(m);
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shape.intersect(ray).filter(|i| i.t >= 0.0)
    }
}
