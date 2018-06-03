use std::mem;

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
                xyrect(min_x, min_y, max_x, max_y, min_z, true),
                xyrect(min_x, min_y, max_x, max_y, max_z, false),
                xzrect(min_x, min_z, max_x, max_z, min_y, true,),
                xzrect(min_x, min_z, max_x, max_z, max_y, false),
                zyrect(min_z, min_y, max_z, max_y, min_x, true),
                zyrect(min_z, min_y, max_z, max_y, max_x, false),
            ],
        }
    }
}

fn xyrect(x0: f64, y0: f64, x1: f64, y1: f64, z: f64, reverse_normal: bool) -> Box<Shape> {
    Box::new(XYRectangle::new(
        Point::new((x1 - x0) / 2.0 + x0, (y1 - y0) / 2.0 + y0, z),
        x1 - x0,
        y1 - y0,
        reverse_normal,
    ))
}

fn xzrect(x0: f64, z0: f64, x1: f64, z1: f64, y: f64, reverse_normal: bool) -> Box<Shape> {
    Box::new(XZRectangle::new(
        Point::new((x1 - x0) / 2.0 + x0, y, (z1 - z0) / 2.0 + z0),
        x1 - x0,
        z1 - z0,
        reverse_normal,
    ))
}

fn zyrect(z0: f64, y0: f64, z1: f64, y1: f64, x: f64, reverse_normal: bool) -> Box<Shape> {
    Box::new(ZYRectangle::new(
        Point::new(x, (y1 - y0) / 2.0 + y0, (z1 - z0) / 2.0 + z0),
        z1 - z0,
        y1 - y0,
        reverse_normal,
    ))
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

impl Shape for Cube {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let mut is: Vec<Intersection> = self.sides.iter().flat_map(|s| s.intersect(ray)).collect();
        if is.len() > 2 {
            panic!("more than two intersections for cube");
        } else if is.len() == 2 {
            let mut a = is.pop().unwrap();
            let mut b = is.pop().unwrap();
            if a > b {
                mem::swap(&mut a, &mut b);
            }
            b.n *= -1.0;
            vec![Interval(a, b)]
        } else if is.len() == 1 {
            let i = is.pop().unwrap();
            vec![Interval(i.clone(), i.clone())]
        } else {
            Vec::with_capacity(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use direction::*;
    use test_utils::*;

    #[test]
    pub fn outside_intersection() {
        let s = Cube::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 2.0), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s.intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a,b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections
            .iter()
            .map(|i| i.t)
            .collect();
        let normals: Vec<Direction> = intersections
            .iter()
            .map(|i| i.n)
            .collect();
        assert_approx_eq(&distances, &vec![
            1.0,
            3.0,
        ]);
        assert_approx_eq(&normals, &vec![
            Direction::new(0.0, 0.0, 1.0),
            Direction::new(0.0, 0.0, -1.0),
        ]);
    }

    #[test]
    pub fn coincident_intersection() {
        let s = Cube::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s.intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a,b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections
            .iter()
            .map(|i| i.t)
            .collect();
        let normals: Vec<Direction> = intersections
            .iter()
            .map(|i| i.n)
            .collect();
        assert_approx_eq(&distances, &vec![
            0.0,
            2.0,
        ]);
        assert_approx_eq(&normals, &vec![
            Direction::new(0.0, 0.0, 1.0),
            Direction::new(0.0, 0.0, -1.0),
        ]);
    }

    #[test]
    pub fn inside_intersection() {
        let s = Cube::new(Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 0.9), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s.intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a,b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections
            .iter()
            .map(|i| i.t)
            .collect();
        let normals: Vec<Direction> = intersections
            .iter()
            .map(|i| i.n)
            .collect();
        assert_approx_eq(&distances, &vec![
            -0.1,
            1.9,
        ]);
        assert_approx_eq(&normals, &vec![
            Direction::new(0.0, 0.0, 1.0),
            Direction::new(0.0, 0.0, -1.0),
        ]);
    }
}
