use crate::materials::Material;
use crate::matrix::Matrix44f;
use crate::shapes::Shape;
use crate::system::{Intersection, Ray, Intersectable, Transformable};

#[derive(Clone)]
pub struct Transformation {
    pub object_to_world: Matrix44f,
    pub world_to_object: Matrix44f,
    pub normal_to_world: Matrix44f,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            object_to_world: Matrix44f::identity(),
            world_to_object: Matrix44f::identity(),
            normal_to_world: Matrix44f::identity(),
        }
    }
}

impl Transformable for Transformation {
    fn transform(&mut self, m: Matrix44f) {
        self.object_to_world = self.object_to_world * m;
        self.world_to_object = self.object_to_world.inverse();
        self.normal_to_world = self.world_to_object.transposed();
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
        let tx = self.shape.transformation();
        let object_ray = ray.to_object(tx);
        self.shape.intersect(&object_ray).and_then(|i| {
            if i.t < 0.0 {
                None
            } else {
                Some(i.to_world(ray, &object_ray, tx))
            }
        })
    }
}
