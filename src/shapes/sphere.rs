use std::f64;
use std::mem;

use direction::Dot;
use shapes::Shape;
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

pub struct Sphere {
    radius_squared: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {
            radius_squared: radius.powi(2),
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
        let l = ray.origin.to_dir();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius_squared;
        if let Some((mut t0, mut t1)) = solve_quadratic(a, b, c) {
            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }
            if t0 < 0.0 {
                t0 = t1;
                if t0 < 0.0 {
                    return None;
                }
            }

            let p = ray.origin + ray.direction * t0;
            let n = (p.to_dir()).normalize();
            let u = (1.0 - n.z.atan2(n.x) / f64::consts::PI) * 0.5;
            let v = n.y.acos() / f64::consts::PI;

            Some(Intersection {
                t: t0,
                n,
                uv: Vector2f(u, v),
            })
        } else {
            None
        }
    }
}

impl Shape for Sphere {}
