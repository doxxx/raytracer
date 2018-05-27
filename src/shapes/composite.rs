use shapes::Shape;
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
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        self.shapes.intersect(ray)
    }
}

impl Shape for Composite {}
