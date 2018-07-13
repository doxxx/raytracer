use std::f64;
use std::mem;

use direction::Direction;
use matrix::Matrix44f;
use object::Transformation;
use point::Point;
use shapes::{Interval, Plane, Shape};
use system::{Intersectable, Intersection, Ray, Transformable};
use vector::Vector2f;

pub struct Cylinder {
    radius: f64,
    height: f64,
    tx: Transformation,
}

impl Cylinder {
    pub fn new(radius: f64, height: f64) -> Cylinder {
        Cylinder {
            radius,
            height,
            tx: Transformation::new(),
        }
    }

    fn side_intersection(&self, o: Point, d: Direction, t: f64, y: f64) -> Intersection {
        let p = o + d * t;
        let n = Direction::new(p.x, 0.0, p.z).normalize();
        let u = (1.0 - n.z.atan2(n.x) / f64::consts::PI) * 0.5;
        let max_y = self.height / 2.0;
        let v = 1.0 - (y + max_y) / self.height;

        Intersection {
            t,
            n,
            uv: Vector2f(u, v),
        }
    }
}

impl Intersectable for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

impl Shape for Cylinder {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let object_ray = ray.to_object(&self.tx);
        let o = object_ray.origin;
        let d = object_ray.direction;
        let a = d.x.powi(2) + d.z.powi(2);
        let b = 2.0 * o.x * d.x + 2.0 * o.z * d.z;
        let c = o.x.powi(2) + o.z.powi(2) - self.radius.powi(2);
        let discr = b.powi(2) - 4.0 * a * c;
        if discr < 0.0 {
            return Vec::with_capacity(0);
        }

        let sqrt = discr.sqrt();
        let mut t0 = (-b + sqrt) / (2.0 * a);
        let mut t1 = (-b - sqrt) / (2.0 * a);

        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }

        let y0 = o.y + t0 * d.y;
        let y1 = o.y + t1 * d.y;

        // figure out which sides have been hit
        let max_y = self.height / 2.0;
        let min_y = -max_y;
        let top_cap = Plane::new(Point::new(0.0, max_y, 0.0), Direction::new(0.0, 1.0, 0.0));
        let bottom_cap = Plane::new(Point::new(0.0, min_y, 0.0), Direction::new(0.0, -1.0, 0.0));

        if y0 < min_y && y1 < min_y || y0 > max_y && y1 > max_y {
            // miss
            Vec::with_capacity(0)
        } else if y0 > max_y {
            // top cap
            let Interval(top_i, _) = top_cap
                .intersection_intervals(&object_ray)
                .pop()
                .expect("expected one interval from top cap intersection");
            if y1 > min_y && y1 < max_y {
                // and back side
                vec![Interval(top_i, self.side_intersection(o, d, t1, y1)).to_world(ray, &object_ray, &self.tx)]
            } else {
                // and bottom cap
                assert!(y1 < min_y);
                let Interval(bottom_i, _) = bottom_cap
                    .intersection_intervals(&object_ray)
                    .pop()
                    .expect("expected one interval from bottom cap intersection");
                vec![Interval(top_i, bottom_i).to_world(ray, &object_ray, &self.tx)]
            }
        } else if y0 < min_y {
            // bottom cap
            let Interval(bottom_i, _) = bottom_cap
                .intersection_intervals(&object_ray)
                .pop()
                .expect("expected one interval from bottom cap intersection");
            if y1 > min_y && y1 < max_y {
                // and back side
                vec![Interval(bottom_i, self.side_intersection(o, d, t1, y1)).to_world(ray, &object_ray, &self.tx)]
            } else {
                // and top cap
                assert!(y1 > max_y);
                let Interval(top_i, _) = top_cap
                    .intersection_intervals(&object_ray)
                    .pop()
                    .expect("expected one interval from top cap intersection");
                vec![Interval(bottom_i, top_i).to_world(ray, &object_ray, &self.tx)]
            }
        } else {
            // front side
            let front_i = self.side_intersection(o, d, t0, y0);
            if y1 > min_y && y1 < max_y {
                // and back side
                vec![Interval(front_i, self.side_intersection(o, d, t1, y1)).to_world(ray, &object_ray, &self.tx)]
            } else if y1 < min_y {
                // and bottom cap
                let Interval(bottom_i, _) = bottom_cap
                    .intersection_intervals(&object_ray)
                    .pop()
                    .expect("expected one interval from bottom cap intersection");
                vec![Interval(front_i, bottom_i).to_world(ray, &object_ray, &self.tx)]
            } else {
                // and top cap
                assert!(y1 > max_y);
                let Interval(top_i, _) = top_cap
                    .intersection_intervals(&object_ray)
                    .pop()
                    .expect("expected one interval from top cap intersection");
                vec![Interval(front_i, top_i).to_world(ray, &object_ray, &self.tx)]
            }
        }
    }
}
