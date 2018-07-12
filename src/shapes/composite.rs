use matrix::Matrix44f;
use object::Transformation;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray};

pub struct Composite {
    shapes: Vec<Box<dyn Shape>>,
}

impl Composite {
    pub fn new(shapes: Vec<Box<dyn Shape>>) -> Composite {
        Composite { shapes }
    }
}

impl Intersectable for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes.intersect(ray)
    }
}

impl Shape for Composite {
    fn transform(&mut self, m: Matrix44f) {
        for s in &mut self.shapes {
            s.transform(m);
        }
    }
    
    fn transformation(&self) -> &Transformation {
        self.shapes[0].transformation()
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let mut is: Vec<Interval> = self.shapes.iter().flat_map(|s| s.intersection_intervals(ray)).collect();
        is.sort_by(|a, b| a.partial_cmp(b).unwrap());
        is
    }
}
