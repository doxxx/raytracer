use shapes::Shape;
use system::{Intersectable, Intersection, Ray};

pub struct Composite {
    shapes: Vec<Box<Shape>>,
}

impl Composite {
    pub fn new(shapes: Vec<Box<Shape>>) -> Composite {
        Composite {
            shapes,
        }
    }
}

impl Intersectable for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes.iter()
            .map(|s| s.intersect(ray))
            .find(|i| i.is_some())
            .unwrap_or_default()
    }
}

impl Shape for Composite {}
