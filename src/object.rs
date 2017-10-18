use materials::Material;
use matrix::Matrix44f;
use shapes::Shape;
use shapes::bounding_box::BoundingBox;
use system::{Intersection, Ray, Intersectable, Transformable};

pub struct Object {
    pub name: String,
    pub shape: Box<Shape>,
    pub material: Box<Material>,
    object_to_world: Matrix44f,
    world_to_object: Matrix44f,
}

impl Object {
    pub fn new(name: &str, shape: Box<Shape>, material: Box<Material>) -> Object {
        Object {
            name: String::from(name),
            shape,
            material,
            object_to_world: Matrix44f::identity(),
            world_to_object: Matrix44f::identity(),
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.shape.bounding_box(self.object_to_world)
    }
}

impl Transformable for Object {
    fn transform(self, m: Matrix44f) -> Self {
        let object_to_world = self.object_to_world * m;
        Object {
            name: self.name,
            shape: self.shape,
            material: self.material,
            object_to_world,
            world_to_object: object_to_world.inverse()
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
                t: (world_hit_point - ray.origin).length_squared().sqrt(),
                n: i.n * self.object_to_world.inverse().transposed(),
                uv: i.uv,
            }
        })
    }
}
