use direction::{Dot, Direction};
use shapes::Shape;
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

pub struct Plane {
    normal: Direction,
    u: Direction,
    v: Direction,
}

impl Plane {
    pub fn new(normal: Direction) -> Plane {
        let mut u = normal.cross(Direction::new(1.0, 0.0, 0.0));
        if u.length_squared() < 1e-6 {
            u = normal.cross(Direction::new(0.0, 1.0, 0.0));
        }
        if u.length_squared() < 1e-6 {
            u = normal.cross(Direction::new(0.0, 0.0, 1.0));
        }
        u = u.normalize();
        let v = normal.cross(u);
        Plane {
            normal,
            u,
            v,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let w = ray.origin.to_dir();
        let t = -self.normal.dot(w) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;
        let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
        Some(Intersection {
            t,
            n: self.normal,
            uv,
        })
    }
}

impl Shape for Plane {}
