use system::Intersection;
use system::Ray;
use system::Intersectable;

pub mod bounding_box;
pub mod cube;
pub mod composite;
pub mod mesh;
pub mod plane;
pub mod sphere;

pub use self::bounding_box::*;
pub use self::cube::*;
pub use self::composite::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::sphere::*;

pub trait Shape: Intersectable + Send + Sync {}

impl Intersectable for [Box<Shape>] {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest: Option<Intersection> = None;

        for s in self {
            if let Some(i) = s.intersect(ray) {
                if nearest.is_none() || i.t < nearest.as_ref().unwrap().t {
                    nearest = Some(i);
                }
            }
        }
        
        nearest
    }
}
