use direction::{Direction, Dot};
use shapes::Shape;
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

fn plane_uv(n: Direction) -> (Direction, Direction) {
    let mut u = n.cross(Direction::new(1.0, 0.0, 0.0));
    if u.length_squared() < 1e-6 {
        u = n.cross(Direction::new(0.0, 1.0, 0.0));
    }
    if u.length_squared() < 1e-6 {
        u = n.cross(Direction::new(0.0, 0.0, 1.0));
    }
    u = u.normalize();
    let v = n.cross(u);

    (u, v)
}

fn plane_intersect(n: Direction, ray: &Ray) -> Option<f64> {
    let denom = n.dot(ray.direction);
    if denom.abs() < 1e-6 {
        return None;
    }
    let w = ray.origin.to_dir();
    let t = -n.dot(w) / denom;
    if t < 0.0 {
        return None;
    }
    Some(t)
}

pub struct Plane {
    normal: Direction,
    u: Direction,
    v: Direction,
}

impl Plane {
    pub fn new(normal: Direction) -> Plane {
        let (u, v) = plane_uv(normal);
        Plane { normal, u, v }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(t) = plane_intersect(self.normal, ray) {
            let p = ray.origin + ray.direction * t;
            let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
            return Some(Intersection { t, n: self.normal, uv });
        }

        None
    }
}

impl Shape for Plane {}

fn xy_normal() -> Direction { Direction::new(0.0, 0.0, 1.0) }

pub struct XYRectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    u: Direction,
    v: Direction,
}

impl XYRectangle {
    pub fn new(width: f64, height: f64) -> XYRectangle {
        let (u, v) = plane_uv(xy_normal());
        let x0 = -width / 2.0;
        let x1 = width / 2.0;
        let y0 = -height / 2.0;
        let y1 = height / 2.0;

        XYRectangle { x0, x1, y0, y1, u, v }
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = xy_normal();
        if let Some(t) = plane_intersect(n, ray) {
            let p = ray.origin + ray.direction * t;
            if p.x < self.x0 || p.x > self.x1 || p.y < self.y0 || p.y > self.y1 {
                return None;
            }
            let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for XYRectangle {}
