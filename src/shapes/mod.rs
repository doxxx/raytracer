use system::Intersectable;
use system::Intersection;
use system::Ray;

pub mod bounding_box;
pub mod composite;
pub mod csg;
pub mod cube;
pub mod mesh;
pub mod plane;
pub mod sphere;

pub use self::bounding_box::*;
pub use self::composite::*;
pub use self::csg::*;
pub use self::cube::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::sphere::*;

pub trait Shape: Intersectable + Send + Sync {}

impl Intersectable for [Box<Shape>] {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        if self.len() == 0 {
            return None;
        }

        let mut all: Vec<Intersection> = self
            .iter()
            .filter_map(|shape| shape.intersect(ray))
            .flat_map(|intersections| intersections)
            .collect();

        if all.len() > 0 {
            all.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(all)
        } else {
            None
        }
    }
}
