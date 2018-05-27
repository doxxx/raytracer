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

fn plane_intersect(o: Point, n: Direction, ray: &Ray) -> Option<f64> {
    let denom = n.dot(ray.direction);
    if denom.abs() < 1e-6 {
        return None;
    }
    let w = ray.origin - o;
    let ndotw = n.dot(w);
    if ndotw < 0.0 {
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
    reverse_normal: Direction,
    bidi: bool,
    uv: (Direction, Direction),
    reverse_uv: (Direction, Direction),
}

impl Plane {
    pub fn new(origin: Point, normal: Direction, bidi: bool) -> Plane {
        let reverse_normal = normal * -1.0;
        let uv = plane_uv(normal);
        let reverse_uv = plane_uv(reverse_normal);
        Plane {
            origin,
            normal,
            reverse_normal,
            bidi,
            uv,
            reverse_uv,
        }
    }

    fn bidi_intersect_with_bounds<F>(&self, ray: &Ray, out_of_bounds: F) -> Option<Intersection> 
        where F: FnOnce(&Point) -> bool 
    {
        let mut n = self.normal;
        let mut uv = self.uv;
        let mut t = plane_intersect(self.origin, n, ray);
        if self.bidi && t.is_none() {
            n = self.reverse_normal;
            uv = self.reverse_uv;
            t = plane_intersect(self.origin, n, ray);
        }
        if let Some(t) = t {
            let p = ray.origin + ray.direction * t;
            if out_of_bounds(&p) {
                return None;
            }
            let op = p - self.origin;
            let uv = Vector2f(uv.0.dot(op), uv.1.dot(op));
            return Some(Intersection { t, n, uv });
        }

        None
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        self.bidi_intersect_with_bounds(ray, |_| false).map(|i| vec![i])
    }
}

impl Shape for Plane {}

pub struct XYRectangle {
    plane: Plane,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

impl XYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> XYRectangle {
        let plane = Plane::new(origin, Direction::new(0.0, 0.0, 1.0), bidi);
        let x0 = origin.x - width / 2.0;
        let x1 = origin.x + width / 2.0;
        let y0 = origin.y - height / 2.0;
        let y1 = origin.y + height / 2.0;

        XYRectangle {
            plane,
            x0,
            x1,
            y0,
            y1,
        }
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        self.plane.bidi_intersect_with_bounds(ray, |p| p.x < self.x0 || p.x > self.x1 || p.y < self.y0 || p.y > self.y1).map(|i| vec![i])
    }
}

impl Shape for XYRectangle {}

pub struct XZRectangle {
    plane: Plane,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
}

impl XZRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> XZRectangle {
        let plane = Plane::new(origin, Direction::new(0.0, 1.0, 0.0), bidi);
        let x0 = origin.x - (width / 2.0);
        let x1 = origin.x + width / 2.0;
        let z0 = origin.z - (height / 2.0);
        let z1 = origin.z + height / 2.0;

        XZRectangle {
            plane,
            x0,
            x1,
            z0,
            z1,
        }
    }
}

impl Intersectable for XZRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        self.plane.bidi_intersect_with_bounds(ray, |p| p.x < self.x0 || p.x > self.x1 || p.z < self.z0 || p.z > self.z1).map(|i| vec![i])
    }
}

impl Shape for XZRectangle {}

pub struct ZYRectangle {
    plane: Plane,
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
}

impl ZYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, bidi: bool) -> ZYRectangle {
        let plane = Plane::new(origin, Direction::new(1.0, 0.0, 0.0), bidi);
        let z0 = origin.z - (width / 2.0);
        let z1 = origin.z + width / 2.0;
        let y0 = origin.y - (height / 2.0);
        let y1 = origin.y + height / 2.0;

        ZYRectangle {
            plane,
            z0,
            z1,
            y0,
            y1,
        }
    }
}

impl Intersectable for ZYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        self.plane.bidi_intersect_with_bounds(ray, |p| p.z < self.z0 || p.z > self.z1 || p.y < self.y0 || p.y > self.y1).map(|i| vec![i])
    }
}

impl Shape for ZYRectangle {}
