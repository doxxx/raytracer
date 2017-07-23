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
        let t = Vector2f(
            ((1.0 + n.2.atan2(n.0)) / f64::consts::PI) * 0.5,
            n.1.acos() / f64::consts::PI,
        );
        (n, t)
    }
}

#[derive(Debug)]
pub struct Plane {
    point: Vector3f,
    normal: Vector3f,
}

impl Plane {
    pub fn new(point: Vector3f, normal: Vector3f) -> Plane {
        Plane {
            point: point,
            normal: normal
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<f64> {
        let denom = self.normal.dot(direction);
        if denom.abs() > 1e-6 {
            let w = origin - self.point;
            let t = -self.normal.dot(w) / denom;
            if t >= 0.0 {
                Some(t)
            }
            else {
                None
            }
        } else {
            None
        }
    }

    fn surface_data(&self, point: Vector3f) -> (Vector3f, Vector2f) {
        let mut u = self.normal.cross(Vector3f(1.0, 0.0, 0.0));
        if u.length_squared() < 1e-6 {
            u = self.normal.cross(Vector3f(0.0, 1.0, 0.0));
        }
        if u.length_squared() < 1e-6 {
            u = self.normal.cross(Vector3f(0.0, 0.0, 1.0));
        }
        u = u.normalize();
        let v = self.normal.cross(u);
        let t = Vector2f(u.dot(point-self.point), v.dot(point-self.point));
        (self.normal, t)
    }
}
