use std::f64;
use std::mem;

use direction::Dot;
use matrix::Matrix44f;
use object::Transformation;
use point::Point;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray, Transformable};
use vector::Vector2f;

pub struct Sphere {
    origin: Point,
    radius_squared: f64,
    tx: Transformation,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Sphere {
        Sphere {
            origin,
            radius_squared: radius.powi(2),
            tx: Transformation::new(),
        }
    }

    fn intersection_for_t(&self, ray: &Ray, t: f64) -> Intersection {
        let p = ray.origin + ray.direction * t;
        let n = (p - self.origin).normalize();
        let u = (1.0 - n.z.atan2(n.x) / f64::consts::PI) * 0.5;
        let v = n.y.acos() / f64::consts::PI;

        Intersection {
            t,
            n,
            uv: Vector2f(u, v),
        }
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return None;
    } else if discr == 0.0 {
        let x = -0.5 * b / a;
        return Some((x, x));
    } else {
        let q = if b > 0.0 {
            -0.5 * (b + discr.sqrt())
        } else {
            -0.5 * (b - discr.sqrt())
        };
        Some((q / a, c / q))
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

impl Shape for Sphere {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn transformation(&self) -> &Transformation {
        &self.tx
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let l = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius_squared;

        if let Some((mut t0, mut t1)) = solve_quadratic(a, b, c) {
            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }

            vec![Interval(
                self.intersection_for_t(ray, t0),
                self.intersection_for_t(ray, t1),
            )]
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
        let s = Sphere::new(Point::zero(), 1.0);
        let r = Ray::primary(Point::new(0.0, 0.0, 2.0), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = intersections.iter().map(|i| i.n).collect();
        assert_approx_eq!(&distances, &vec![1.0, 3.0]);
        assert_approx_eq!(
            &normals,
            &vec![Direction::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0)]
        );
    }

    #[test]
    pub fn coincident_intersection() {
        let s = Sphere::new(Point::zero(), 1.0);
        let r = Ray::primary(Point::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = intersections.iter().map(|i| i.n).collect();
        assert_approx_eq!(&distances, &vec![0.0, 2.0]);
        assert_approx_eq!(
            &normals,
            &vec![Direction::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0)]
        );
    }

    #[test]
    pub fn inside_intersection() {
        let s = Sphere::new(Point::zero(), 1.0);
        let r = Ray::primary(Point::new(0.0, 0.0, 0.9), Direction::new(0.0, 0.0, -1.0), 0);
        let intersections: Vec<Intersection> = s
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = intersections.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = intersections.iter().map(|i| i.n).collect();
        assert_approx_eq!(&distances, &vec![-0.1, 1.9]);
        assert_approx_eq!(
            &normals,
            &vec![Direction::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0)]
        );
    }
}
