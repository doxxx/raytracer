use matrix::Matrix44f;
use shapes::Shape;
use shapes::bounding_box::BoundingBox;
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

impl Shape for Composite {
    fn bounding_box(&self, m: Matrix44f) -> BoundingBox {
        self.shapes.iter().fold(BoundingBox::zero(), |acc, shape| acc.extend(&shape.bounding_box(m)))
    }
}
