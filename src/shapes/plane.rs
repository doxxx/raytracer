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

fn plane_intersect(n: Direction, ray: &Ray, bidi: bool) -> Option<f64> {
    let denom = n.dot(ray.direction);
    if denom.abs() < 1e-6 {
        return None;
    }
    let w = ray.origin.to_dir();
    let ndotw = n.dot(w);
    if !bidi && ndotw < 0.0 {
        return None;
    }
    let t = -ndotw / denom;
    if t < 0.0 {
        return None;
    }
    Some(t)
}

pub struct Plane {
    normal: Direction,
    bidi: bool,
    u: Direction,
    v: Direction,
}

impl Plane {
    pub fn new(normal: Direction, bidi: bool) -> Plane {
        let (u, v) = plane_uv(normal);
        Plane { normal, bidi, u, v }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(t) = plane_intersect(self.normal, ray, self.bidi) {
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
    bidi: bool,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    u: Direction,
    v: Direction,
}

impl XYRectangle {
    pub fn new(width: f64, height: f64, bidi: bool) -> XYRectangle {
        let (u, v) = plane_uv(xy_normal());
        let x0 = -(width / 2.0);
        let x1 = width / 2.0;
        let y0 = -(height / 2.0);
        let y1 = height / 2.0;

        XYRectangle { bidi, x0, x1, y0, y1, u, v }
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = xy_normal();
        if let Some(t) = plane_intersect(n, ray, self.bidi) {
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

fn xz_normal() -> Direction { Direction::new(0.0, 1.0, 0.0) }

pub struct XZRectangle {
    bidi: bool,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    u: Direction,
    v: Direction,
}

impl XZRectangle {
    pub fn new(width: f64, height: f64, bidi: bool) -> XZRectangle {
        let (u, v) = plane_uv(xz_normal());
        let x0 = -(width / 2.0);
        let x1 = width / 2.0;
        let z0 = -(height / 2.0);
        let z1 = height / 2.0;

        XZRectangle { bidi, x0, x1, z0, z1, u, v }
    }
}

impl Intersectable for XZRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = xz_normal();
        if let Some(t) = plane_intersect(n, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            if p.x < self.x0 || p.x > self.x1 || p.z < self.z0 || p.z > self.z1 {
                return None;
            }
            let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for XZRectangle {}

fn zy_normal() -> Direction { Direction::new(1.0, 0.0, 0.0) }

pub struct ZYRectangle {
    bidi: bool,
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
    u: Direction,
    v: Direction,
}

impl ZYRectangle {
    pub fn new(width: f64, height: f64, bidi: bool) -> ZYRectangle {
        let (u, v) = plane_uv(zy_normal());
        let z0 = -(width / 2.0);
        let z1 = width / 2.0;
        let y0 = -(height / 2.0);
        let y1 = height / 2.0;

        ZYRectangle { bidi, z0, z1, y0, y1, u, v }
    }
}

impl Intersectable for ZYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = zy_normal();
        if let Some(t) = plane_intersect(n, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            if p.z < self.z0 || p.z > self.z1 || p.y < self.y0 || p.y > self.y1 {
                return None;
            }
            let uv = Vector2f(self.u.dot(p.to_dir()), self.v.dot(p.to_dir()));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for ZYRectangle {}
