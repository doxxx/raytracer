use std::f64;
use std::mem;

use matrix::Matrix44f;
use system::{Intersection, Ray};
use vector::{Vector2f, Vector3f};

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    bounds: [Vector3f; 2],
}

impl BoundingBox {
    pub fn new(min: Vector3f, max: Vector3f) -> BoundingBox {
        BoundingBox { bounds: [min, max] }
    }

    pub fn intersect(&self, ray: Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].0 - ray.origin.0) * ray.inverse_direction.0;
        let mut tmax = (self.bounds[1 - ray.sign[0]].0 - ray.origin.0) * ray.inverse_direction.0;
        let tymin = (self.bounds[ray.sign[1]].1 - ray.origin.1) * ray.inverse_direction.1;
        let tymax = (self.bounds[1 - ray.sign[1]].1 - ray.origin.1) * ray.inverse_direction.1;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].2 - ray.origin.2) * ray.inverse_direction.2;
        let tzmax = (self.bounds[1 - ray.sign[2]].2 - ray.origin.2) * ray.inverse_direction.2;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        // if tzmin > tmin {
        //     tmin = tzmin;
        // }
        // if tzmax < tmax {
        //     tmax = tzmax;
        // }

        return true;
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle),
}

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(&self, m: Matrix44f) -> Self;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vector3f,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {
            center: Vector3f::zero(),
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
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let L = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(L);
        let c = L.dot(L) - self.radius_squared;
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
            let n = (p - self.center).normalize();

            Some(Intersection {
                t: t0,
                n: n,
                uv: Vector2f(0.0, 0.0),
            })
        } else {
            None
        }
    }
}

impl Transformable for Sphere {
    fn transform(&self, m: Matrix44f) -> Self {
        let center = self.center * m;
        let radius_point = (self.center + Vector3f(self.radius_squared.sqrt(), 0.0, 0.0)) * m;
        Sphere {
            center: center,
            radius_squared: (radius_point - center).length_squared(),
        }
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
    pub fn new(normal: Vector3f) -> Plane {
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
            point: Vector3f::zero(),
            normal: normal,
            u: u,
            v: v,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let w = ray.origin - self.point;
        let t = -self.normal.dot(w) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;
        let uv = Vector2f(self.u.dot(p - self.point), self.v.dot(p - self.point));
        Some(Intersection {
            t: t,
            n: self.normal,
            uv: uv,
        })
    }
}

impl Transformable for Plane {
    fn transform(&self, m: Matrix44f) -> Self {
        let mut p = Plane::new(
            m.inverse().transposed().mult_normal(self.normal)
        );
        p.point = self.point * m;
        p
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

impl Intersectable for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.normal.dot(self.normal);

        let normal_dot_ray = self.normal.dot(ray.direction);
        if normal_dot_ray.abs() < 1e-6 {
            return None;
        }

        let d = self.normal.dot(self.vertices[0]);
        let t = (self.normal.dot(ray.origin) + d) / normal_dot_ray;
        if t < 0.0 {
            return None;
        }

        let p = ray.origin + ray.direction * t;

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

impl Transformable for Triangle {
    fn transform(&self, m: Matrix44f) -> Self {
        let v0 = self.vertices[0] * m;
        let v1 = self.vertices[1] * m;
        let v2 = self.vertices[2] * m;
        Triangle::new(v0, v1, v2)
    }
}
