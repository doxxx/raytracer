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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Interval(Intersection, Intersection);

pub trait Shape: Intersectable + Send + Sync {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval>;
}

impl Intersectable for [Box<Shape>] {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.len() == 0 {
            return None;
        }

        self.iter()
            .flat_map(|s| s.intersect(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}
