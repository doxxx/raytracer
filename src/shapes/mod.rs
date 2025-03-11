use crate::matrix::Matrix44f;
use crate::object::Transformation;
use crate::system::{Intersectable, Intersection, Ray};

mod bounding_box;
mod composite;
mod csg;
mod cube;
mod cylinder;
mod homogenous_medium;
mod mesh;
mod plane;
mod sphere;
mod torus;

pub use self::bounding_box::*;
pub use self::composite::*;
pub use self::csg::*;
pub use self::cube::*;
pub use self::cylinder::*;
pub use self::homogenous_medium::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::sphere::*;
pub use self::torus::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval(Intersection, Intersection);

impl Interval {
    pub fn to_world(self, world_ray: &Ray, object_ray: &Ray, tx: &Transformation) -> Interval {
        Interval(
            self.0.to_world(world_ray, &object_ray, tx),
            self.1.to_world(world_ray, &object_ray, tx),
        )
    }
}

pub fn skip_negative_intervals(intervals: Vec<Interval>) -> impl Iterator<Item = Interval> {
    intervals
        .into_iter()
        .skip_while(|Interval(a, b)| a.t < 0.0 && b.t < 0.0)
}

pub fn first_positive_intersection(intervals: Vec<Interval>) -> Option<Intersection> {
    intervals
        .into_iter()
        .flat_map(|Interval(a, b)| vec![a, b])
        .skip_while(|i| i.t < 0.0)
        .nth(0)
}

pub fn first_intersection(intervals: Vec<Interval>) -> Option<Intersection> {
    intervals.into_iter().nth(0).map(|i| i.0)
}

pub trait Shape: Intersectable + Send + Sync {
    fn transform(&mut self, m: Matrix44f);
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval>;
}

impl Intersectable for [Box<dyn Shape>] {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.len() == 0 {
            return None;
        }

        self.iter()
            .flat_map(|s| s.intersect(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}
