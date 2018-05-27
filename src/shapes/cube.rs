use super::{XYRectangle, XZRectangle, ZYRectangle};
use point::Point;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray};

pub struct Cube {
    sides: [Box<Shape>; 6],
}

impl Cube {
    pub fn new(p1: Point, p2: Point) -> Cube {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let min_z = p1.z.min(p2.z);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        let max_z = p1.z.max(p2.z);
        Cube {
            sides: [
                xyrect(min_x, min_y, max_x, max_y, min_z),
                xyrect(min_x, min_y, max_x, max_y, max_z),
                xzrect(min_x, min_z, max_x, max_z, min_y),
                xzrect(min_x, min_z, max_x, max_z, max_y),
                zyrect(min_z, min_y, max_z, max_y, min_x),
                zyrect(min_z, min_y, max_z, max_y, max_x),
            ],
        }
    }
}

fn xyrect(x0: f64, y0: f64, x1: f64, y1: f64, z: f64) -> Box<Shape> {
    Box::new(XYRectangle::new(
        Point::new((x1 - x0) / 2.0 + x0, (y1 - y0) / 2.0 + y0, z),
        x1 - x0,
        y1 - y0,
        true,
    ))
}

fn xzrect(x0: f64, z0: f64, x1: f64, z1: f64, y: f64) -> Box<Shape> {
    Box::new(XZRectangle::new(
        Point::new((x1 - x0) / 2.0 + x0, y, (z1 - z0) / 2.0 + z0),
        x1 - x0,
        z1 - z0,
        true,
    ))
}

fn zyrect(z0: f64, y0: f64, z1: f64, y1: f64, x: f64) -> Box<Shape> {
    Box::new(ZYRectangle::new(
        Point::new(x, (y1 - y0) / 2.0 + y0, (z1 - z0) / 2.0 + z0),
        z1 - z0,
        y1 - y0,
        true,
    ))
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.sides.intersect(ray)
    }
}

impl Shape for Cube {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let mut is: Vec<Intersection> = self.sides.iter().flat_map(|s| s.intersect(ray)).collect();
        if is.len() == 0 {
            return Vec::new();
        } else {
            assert!(is.len() == 2);
            vec![Interval(is.pop().unwrap(), is.pop().unwrap())]
        }
    }
}
