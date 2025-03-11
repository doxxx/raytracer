use crate::matrix::Matrix44f;
use crate::object::Transformation;
use crate::shapes::{Interval, Shape};
use crate::system::{Intersectable, Intersection, Ray, Transformable};

pub struct Composite {
    shapes: Vec<Box<dyn Shape>>,
    tx: Transformation,
}

impl Composite {
    pub fn new(shapes: Vec<Box<dyn Shape>>) -> Composite {
        Composite { shapes, tx: Transformation::new() }
    }
}

impl Intersectable for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.shapes.len() == 0 {
            return None;
        }

        let object_ray = ray.to_object(&self.tx);

        self.shapes.iter()
            .flat_map(|s| s.intersect(&object_ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .map(|i| i.to_world(ray, &object_ray, &self.tx))
    }
}

impl Shape for Composite {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }
    
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let object_ray = ray.to_object(&self.tx);
        let mut is: Vec<Interval> = self.shapes
            .iter()
            .flat_map(|s| s.intersection_intervals(&object_ray))
            .collect();
        is.sort_by(|a, b| a.partial_cmp(b).unwrap());
        is.into_iter().map(|i| i.to_world(ray, &object_ray, &self.tx)).collect()
    }
}
