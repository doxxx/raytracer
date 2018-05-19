use direction::{Direction, Dot};
use point::Point;
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

fn plane_intersect(o: Point, n: Direction, ray: &Ray, bidi: bool) -> Option<f64> {
    let denom = n.dot(ray.direction);
    if denom.abs() < 1e-6 {
        return None;
    }
    let w = ray.origin - o;
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
    origin: Point,
    normal: Direction,
    bidi: bool,
    u: Direction,
    v: Direction,
}

impl Plane {
    pub fn new(origin: Point, normal: Direction, bidi: bool) -> Plane {
        let (u, v) = plane_uv(normal);
        Plane {
            origin,
            normal,
            bidi,
            u,
            v,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(t) = plane_intersect(self.origin, self.normal, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            let op = p - self.origin;
            let uv = Vector2f(self.u.dot(op), self.v.dot(op));
            return Some(Intersection { t, n: self.normal, uv });
        }

        None
    }
}

impl Shape for Plane {}

fn xy_normal() -> Direction {
    Direction::new(0.0, 0.0, 1.0)
}

pub struct XYRectangle {
    origin: Point,
    bidi: bool,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    u: Direction,
    v: Direction,
}

impl XYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> XYRectangle {
        let (u, v) = plane_uv(xy_normal());
        let x0 = origin.x - width / 2.0;
        let x1 = origin.x + width / 2.0;
        let y0 = origin.y - height / 2.0;
        let y1 = origin.y + height / 2.0;

        XYRectangle {
            origin,
            bidi,
            x0,
            x1,
            y0,
            y1,
            u,
            v,
        }
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = xy_normal();
        if let Some(t) = plane_intersect(self.origin, n, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            if p.x < self.x0 || p.x > self.x1 || p.y < self.y0 || p.y > self.y1 {
                return None;
            }
            let op = p - self.origin;
            let uv = Vector2f(self.u.dot(op), self.v.dot(op));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for XYRectangle {}

fn xz_normal() -> Direction {
    Direction::new(0.0, 1.0, 0.0)
}

pub struct XZRectangle {
    origin: Point,
    bidi: bool,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    u: Direction,
    v: Direction,
}

impl XZRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> XZRectangle {
        let (u, v) = plane_uv(xz_normal());
        let x0 = origin.x - (width / 2.0);
        let x1 = origin.x + width / 2.0;
        let z0 = origin.z - (height / 2.0);
        let z1 = origin.z + height / 2.0;

        XZRectangle {
            origin,
            bidi,
            x0,
            x1,
            z0,
            z1,
            u,
            v,
        }
    }
}

impl Intersectable for XZRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = xz_normal();
        if let Some(t) = plane_intersect(self.origin, n, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            if p.x < self.x0 || p.x > self.x1 || p.z < self.z0 || p.z > self.z1 {
                return None;
            }
            let op = p - self.origin;
            let uv = Vector2f(self.u.dot(op), self.v.dot(op));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for XZRectangle {}

fn zy_normal() -> Direction {
    Direction::new(1.0, 0.0, 0.0)
}

pub struct ZYRectangle {
    origin: Point,
    bidi: bool,
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
    u: Direction,
    v: Direction,
}

impl ZYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> ZYRectangle {
        let (u, v) = plane_uv(zy_normal());
        let z0 = origin.z - (width / 2.0);
        let z1 = origin.z + width / 2.0;
        let y0 = origin.y - (height / 2.0);
        let y1 = origin.y + height / 2.0;

        ZYRectangle {
            origin,
            bidi,
            z0,
            z1,
            y0,
            y1,
            u,
            v,
        }
    }
}

impl Intersectable for ZYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let n = zy_normal();
        if let Some(t) = plane_intersect(self.origin, n, ray, self.bidi) {
            let p = ray.origin + ray.direction * t;
            if p.z < self.z0 || p.z > self.z1 || p.y < self.y0 || p.y > self.y1 {
                return None;
            }
            let op = p - self.origin;
            let uv = Vector2f(self.u.dot(op), self.v.dot(op));
            return Some(Intersection { t, n: n, uv });
        }

        None
    }
}

impl Shape for ZYRectangle {}
