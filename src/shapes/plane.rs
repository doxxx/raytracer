use direction::{Direction, Dot};
use matrix::Matrix44f;
use object::Transformation;
use point::Point;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray, Transformable};
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
    let denom = ray.direction.dot(n);
    if denom.abs() > 1e-6 {
        let w = o - ray.origin;
        let t = w.dot(n) / denom;
        Some(t)
    } else {
        None
    }
}

pub struct Plane {
    origin: Point,
    normal: Direction,
    reverse_normal: Direction,
    uv: (Direction, Direction),
    reverse_uv: (Direction, Direction),
    tx: Transformation,
}

impl Plane {
    pub fn new(origin: Point, normal: Direction) -> Plane {
        let reverse_normal = normal * -1.0;
        let uv = plane_uv(normal);
        let reverse_uv = plane_uv(reverse_normal);
        Plane {
            origin,
            normal,
            reverse_normal,
            uv,
            reverse_uv,
            tx: Transformation::new(),
        }
    }

    fn intersect_with_bounds<F>(&self, ray: &Ray, out_of_bounds: F) -> Option<Intersection>
    where
        F: FnOnce(Point) -> bool,
    {
        let object_ray = ray.to_object(&self.tx);
        let mut n = self.normal;
        let mut uv = self.uv;
        let t = plane_intersect(self.origin, n, &object_ray);
        if let Some(t) = t {
            if object_ray.direction.dot(self.normal) > 0.0 {
                n = self.reverse_normal;
                uv = self.reverse_uv;
            }
            let p = object_ray.origin + object_ray.direction * t;
            if out_of_bounds(p) {
                return None;
            }
            let op = p - self.origin;
            let uv = Vector2f(uv.0.dot(op), uv.1.dot(op));
            let i = Intersection { t, n, uv };
            let iw = i.to_world(ray, &object_ray, &self.tx);
            return Some(iw);
        }

        None
    }

    fn intersection_intervals_with_bounds<F>(&self, ray: &Ray, out_of_bounds: F) -> Vec<Interval>
    where
        F: FnOnce(Point) -> bool,
    {
        self.intersect_with_bounds(ray, out_of_bounds)
            .map(|i| vec![Interval(i, i.clone())])
            .unwrap_or(Vec::with_capacity(0))
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.intersect_with_bounds(ray, |_| false)
    }
}

impl Shape for Plane {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.intersection_intervals_with_bounds(ray, |_| false)
    }
}

pub struct XYRectangle {
    plane: Plane,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

impl XYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, reverse_normal: bool) -> XYRectangle {
        let mut normal = Direction::new(0.0, 0.0, 1.0);
        if reverse_normal {
            normal *= -1.0;
        }
        let plane = Plane::new(origin, normal);
        let x0 = origin.x - width / 2.0;
        let x1 = origin.x + width / 2.0;
        let y0 = origin.y - height / 2.0;
        let y1 = origin.y + height / 2.0;

        XYRectangle { plane, x0, x1, y0, y1 }
    }

    fn out_of_bounds(&self, p: Point) -> bool {
        p.x < self.x0 || p.x > self.x1 || p.y < self.y0 || p.y > self.y1
    }
}

impl Intersectable for XYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.plane.intersect_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

impl Shape for XYRectangle {
    fn transform(&mut self, m: Matrix44f) {
        self.plane.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.plane
            .intersection_intervals_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

pub struct XZRectangle {
    plane: Plane,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
}

impl XZRectangle {
    pub fn new(origin: Point, width: f64, height: f64, reverse_normal: bool) -> XZRectangle {
        let mut normal = Direction::new(0.0, 1.0, 0.0);
        if reverse_normal {
            normal *= -1.0;
        }
        let plane = Plane::new(origin, normal);
        let x0 = origin.x - (width / 2.0);
        let x1 = origin.x + width / 2.0;
        let z0 = origin.z - (height / 2.0);
        let z1 = origin.z + height / 2.0;

        XZRectangle { plane, x0, x1, z0, z1 }
    }

    fn out_of_bounds(&self, p: Point) -> bool {
        p.x < self.x0 || p.x > self.x1 || p.z < self.z0 || p.z > self.z1
    }
}

impl Intersectable for XZRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.plane.intersect_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

impl Shape for XZRectangle {
    fn transform(&mut self, m: Matrix44f) {
        self.plane.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.plane
            .intersection_intervals_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

pub struct ZYRectangle {
    plane: Plane,
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
}

impl ZYRectangle {
    pub fn new(origin: Point, width: f64, height: f64, reverse_normal: bool) -> ZYRectangle {
        let mut normal = Direction::new(1.0, 0.0, 0.0);
        if reverse_normal {
            normal *= -1.0;
        }
        let plane = Plane::new(origin, normal);
        let z0 = origin.z - (width / 2.0);
        let z1 = origin.z + width / 2.0;
        let y0 = origin.y - (height / 2.0);
        let y1 = origin.y + height / 2.0;

        ZYRectangle { plane, z0, z1, y0, y1 }
    }

    fn out_of_bounds(&self, p: Point) -> bool {
        p.z < self.z0 || p.z > self.z1 || p.y < self.y0 || p.y > self.y1
    }
}

impl Intersectable for ZYRectangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.plane.intersect_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

impl Shape for ZYRectangle {
    fn transform(&mut self, m: Matrix44f) {
        self.plane.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        self.plane
            .intersection_intervals_with_bounds(ray, |p| self.out_of_bounds(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use direction::*;
    use test_utils::*;

    #[test]
    pub fn front_intersection() {
        let s = Plane::new(Point::zero(), Direction::new(0.0, 0.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let i = s.intersect(&r).unwrap();
        assert_approx_eq!(i.t, 1.0);
        assert_approx_eq!(i.n, Direction::new(0.0, 0.0, 1.0));
    }

    #[test]
    pub fn back_intersection() {
        let s = Plane::new(Point::zero(), Direction::new(0.0, 0.0, -1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let i = s.intersect(&r).unwrap();
        assert_approx_eq!(i.t, 1.0);
        assert_approx_eq!(i.n, Direction::new(0.0, 0.0, 1.0));
    }

    #[test]
    pub fn non_intersection() {
        let s = Plane::new(Point::zero(), Direction::new(0.0, 0.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, 1.0), Direction::new(0.0, 1.0, 0.0), 0);
        assert!(s.intersect(&r).is_none());
    }

    #[test]
    pub fn intersection_behind_ray() {
        let s = Plane::new(Point::zero(), Direction::new(0.0, 0.0, 1.0));
        let r = Ray::primary(Point::new(0.0, 0.0, -1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let i = s.intersect(&r).unwrap();
        assert_approx_eq!(i.t, -1.0);
    }
}
