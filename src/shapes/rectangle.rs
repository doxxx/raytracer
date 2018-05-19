use direction::{Direction, Dot};
use shapes::Shape;
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

pub struct XYRectangle {
    width: f64,
    height: f64,
    u: Direction,
    v: Direction,
}

impl XYRectangle {
    pub fn new(width: f64, height: f64) -> XYRectangle {
        let normal = Direction::new(0.0, 0.0, 1.0);
        let mut u = normal.cross(Direction::new(1.0, 0.0, 0.0));
        if u.length_squared() < 1e-6 {
            u = normal.cross(Direction::new(0.0, 1.0, 0.0));
        }
        if u.length_squared() < 1e-6 {
            u = normal.cross(Direction::new(0.0, 0.0, 1.0));
        }
        u = u.normalize();
        let v = normal.cross(u);
        XYRectangle {
            width,
            height,
            u,
            v,
        }
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal = Direction::new(0.0, 0.0, 1.0);
        let denom = normal.dot(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let w = ray.origin.to_dir();
        let t = -normal.dot(w) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;
        let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
        if p.x >= -self.width / 2.0 && p.y >= -self.height / 2.0
            && p.x <= self.width / 2.0 && p.y <= self.height / 2.0
        {
            Some(Intersection { t, n: normal, uv })
        } else {
            None
        }
    }
}

impl Shape for XYRectangle {}
