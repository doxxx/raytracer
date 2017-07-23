use std::f64;
use std::mem;
use vector::{Vector2f, Vector3f};

pub trait Shape {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<f64>;
    fn surface_data(&self, point: Vector3f) -> (Vector3f, Vector2f);
}

pub struct Sphere {
    pub center: Vector3f,
    pub radius: f64,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            radius_squared: radius.powi(2),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<f64> {
        let l = self.center - origin;
        let tca = l.dot(direction);
        if tca < 0f64 {
            return None;
        }
        let d2 = l.dot(l) - tca.powi(2);
        if d2 > self.radius_squared {
            return None;
        }
        let thc = (self.radius_squared - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;
        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }
        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        Some(t0)
    }

    fn surface_data(&self, point: Vector3f) -> (Vector3f, Vector2f) {
        let n = (point - self.center).normalize();
        let t = Vector2f(((1.0 + n.2.atan2(n.0)) / f64::consts::PI) * 0.5,
                         n.1.acos() / f64::consts::PI);
        (n, t)
    }
}
