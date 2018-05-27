use materials::Material;
use matrix::Matrix44f;
use shapes::Shape;
use system::{Intersection, Ray, Intersectable, Transformable};

pub struct Object {
    pub name: String,
    pub shape: Box<Shape>,
    pub material: Box<Material>,
    object_to_world: Matrix44f,
    world_to_object: Matrix44f,
    normal_to_world: Matrix44f,
}

impl Object {
    pub fn new(name: &str, shape: Box<Shape>, material: Box<Material>) -> Object {
        Object {
            name: String::from(name),
            shape,
            material,
            object_to_world: Matrix44f::identity(),
            world_to_object: Matrix44f::identity(),
            normal_to_world: Matrix44f::identity(),
        }
    }
}

impl Transformable for Object {
    fn transform(self, m: Matrix44f) -> Self {
        let object_to_world = self.object_to_world * m;
        let world_to_object = object_to_world.inverse();
        let normal_to_world = world_to_object.transposed();
        Object {
            name: self.name,
            shape: self.shape,
            material: self.material,
            object_to_world,
            world_to_object,
            normal_to_world,
        }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let object_ray = ray.transform(self.world_to_object);
        self.shape.intersect(&object_ray).map(|i| {
            let object_hit_point = i.point(&object_ray);
            let world_hit_point = object_hit_point * self.object_to_world;
            Intersection {
                t: (world_hit_point - ray.origin).length(),
                n: i.n * self.normal_to_world,
                uv: i.uv,
            }
        })
    }
}
