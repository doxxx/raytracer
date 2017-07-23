use std::f64;
use std::fmt::Debug;
use std::mem;

use vector::{Vector2f, Vector3f};

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub n: Vector3f,
    pub uv: Vector2f,
}

pub trait Shape: Debug {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vector3f,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius_squared: radius.powi(2),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<Intersection> {
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

        let p = origin + direction * t0;
        let n = (p - self.center).normalize();
        let uv = Vector2f(
            ((1.0 + n.2.atan2(n.0)) / f64::consts::PI) * 0.5,
            n.1.acos() / f64::consts::PI,
        );

        Some(Intersection {
            t: t0,
            n: n,
            uv: uv,
        })
    }
}

#[derive(Debug)]
pub struct Plane {
    point: Vector3f,
    normal: Vector3f,
    u: Vector3f,
    v: Vector3f,
}

impl Plane {
    pub fn new(point: Vector3f, normal: Vector3f) -> Plane {
        let mut u = normal.cross(Vector3f(1.0, 0.0, 0.0));
        if u.length_squared() < 1e-6 {
            u = normal.cross(Vector3f(0.0, 1.0, 0.0));
        }
        if u.length_squared() < 1e-6 {
            u = normal.cross(Vector3f(0.0, 0.0, 1.0));
        }
        u = u.normalize();
        let v = normal.cross(u);
        Plane {
            point: point,
            normal: normal,
            u: u,
            v: v,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<Intersection> {
        let denom = self.normal.dot(direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let w = origin - self.point;
        let t = -self.normal.dot(w) / denom;
        if t < 0.0 {
            return None;
        }
        let p = origin + direction * t;
        let uv = Vector2f(self.u.dot(p - self.point), self.v.dot(p - self.point));
        Some(Intersection {
            t: t,
            n: self.normal,
            uv: uv,
        })
    }
}

#[derive(Debug)]
pub struct Triangle {
    vertices: [Vector3f; 3],
    edges: [Vector3f; 3],
    normal: Vector3f,
}

impl Triangle {
    pub fn new(v0: Vector3f, v1: Vector3f, v2: Vector3f) -> Triangle {
        Triangle {
            vertices: [v0, v1, v2],
            edges: [v1 - v0, v2 - v1, v0 - v2],
            normal: (v1 - v0).cross(v2 - v0).normalize(),
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, origin: Vector3f, direction: Vector3f) -> Option<Intersection> {
        let denom = self.normal.dot(self.normal);

        let normal_dot_ray = self.normal.dot(direction);
        if normal_dot_ray.abs() < 1e-6 {
            return None;
        }

        let d = self.normal.dot(self.vertices[0]);
        let t = (self.normal.dot(origin) + d) / normal_dot_ray;
        if t < 0.0 {
            return None;
        }

        let p = origin + direction * t;

        let c0 = self.edges[0].cross(p - self.vertices[0]);
        let u = self.normal.dot(c0);
        if u < 0.0 {
            return None;
        }

        let c1 = self.edges[1].cross(p - self.vertices[1]);
        if self.normal.dot(c1) < 0.0 {
            return None;
        }

        let c2 = self.edges[2].cross(p - self.vertices[2]);
        let v = self.normal.dot(c2);
        if v < 0.0 {
            return None;
        }

        Some(Intersection {
            t: t,
            n: self.normal,
            uv: Vector2f(u / denom, v / denom),
        })

    }
}
