use point::Point;
use shapes::{Shape, Interval};
use system::{Intersectable, Intersection, Ray};

pub struct Composite {
    shapes: Vec<Box<Shape>>,
}

impl Composite {
    pub fn new(shapes: Vec<Box<Shape>>) -> Composite {
        Composite { shapes }
    }
}

impl Intersectable for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes.intersect(ray)
    }
}

impl Shape for Composite {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        panic!("not a solid");
    }
}
