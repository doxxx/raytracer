use std::f64;

use rand;
use rand::Rng;

use crate::direction::*;
use crate::matrix::Matrix44f;
use crate::object::Transformation;
use crate::shapes::{Interval, Shape, skip_negative_intervals};
use crate::system::{Intersectable, Intersection, Ray, Transformable};
use crate::vector::Vector2f;

pub struct HomogenousMedium {
    boundary: Box<dyn Shape>,
    density: f64,
    tx: Transformation,
}

impl HomogenousMedium {
    pub fn new(boundary: Box<dyn Shape>, density: f64) -> HomogenousMedium {
        HomogenousMedium {
            boundary,
            density,
            tx: Transformation::new(),
        }
    }
}

impl Intersectable for HomogenousMedium {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let object_ray = ray.to_object(&self.tx);
        let is = self.intersection_intervals(&object_ray);
        if is.len() == 0 {
            return None;
        }

        let mut rng = rand::rng();

        skip_negative_intervals(is)
            .flat_map(|Interval(a, b)| {
                let (at, bt) = (a.t.max(0.0), b.t);
                let distance = ((bt - at) * object_ray.direction).length();
                let hit_distance = -(1.0 / self.density) * rng.random::<f64>().ln();
                if hit_distance < distance {
                    Some(Intersection {
                        t: at + hit_distance / object_ray.direction.length(),
                        n: Direction::new(1.0, 0.0, 0.0),
                        uv: Vector2f(0.0, 0.0),
                    })
                } else {
                    None
                }
            })
            .nth(0)
            .map(|i| i.to_world(ray, &object_ray, &self.tx))
    }
}

impl Shape for HomogenousMedium {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.boundary.intersection_intervals(ray)
    }
}
