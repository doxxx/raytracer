use std::f64;

use rand;
use rand::Rng;

use direction::*;
use shapes::{Interval, Shape, skip_negative_intervals};
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

pub struct HomogenousMedium {
    boundary: Box<Shape>,
    density: f64,
}

impl HomogenousMedium {
    pub fn new(boundary: Box<Shape>, density: f64) -> HomogenousMedium {
        HomogenousMedium { boundary, density }
    }
}

impl Intersectable for HomogenousMedium {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let is = self.intersection_intervals(ray);
        if is.len() == 0 {
            return None;
        }

        let mut rng = rand::thread_rng();

        skip_negative_intervals(is)
            .flat_map(|Interval(a, b)| {
                let (at, bt) = (a.t.max(0.0), b.t);
                let distance = ((bt - at) * ray.direction).length();
                let hit_distance = -(1.0 / self.density) * rng.gen::<f64>().ln();
                if hit_distance < distance {
                    Some(Intersection {
                        t: at + hit_distance / ray.direction.length(),
                        n: Direction::new(1.0, 0.0, 0.0),
                        uv: Vector2f(0.0, 0.0),
                    })
                } else {
                    None
                }
            })
            .nth(0)
    }
}

impl Shape for HomogenousMedium {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.boundary.intersection_intervals(ray)
    }
}
