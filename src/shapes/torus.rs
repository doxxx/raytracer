use std::f64;

use algebra::solve_quartic_f64;
use direction::{Direction, Dot};
use matrix::Matrix44f;
use object::Transformation;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray, Transformable};
use vector::Vector2f;

pub struct Torus {
    radius1: f64,
    radius2: f64,
    tx: Transformation,
}

impl Torus {
    pub fn new(radius1: f64, radius2: f64) -> Torus {
        Torus {
            radius1,
            radius2,
            tx: Transformation::new(),
        }
    }

    fn intersection_for(&self, ray: &Ray, t: f64) -> Intersection {
        let p = ray.origin + ray.direction * t;
        let a = 1.0 - (self.radius1 / (p.x * p.x + p.y * p.y).sqrt());
        let n = Direction::new(a * p.x, a * p.y, p.z).normalize();

        Intersection {
            t,
            n,
            uv: Vector2f(0.0, 0.0),
        }
    }
}

impl Intersectable for Torus {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

impl Shape for Torus {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        /*
        Transcribed from http://cosinekitty.com/raytrace/rtsource.zip.
        Original written by Don Cross.
        Adapted to Rust by Gordon Tyler.
        */

        let object_ray = ray.to_object(&self.tx);
        let o = object_ray.origin;
        let d = object_ray.direction;

        let R = self.radius1;
        let S = self.radius2;

        let T = 4.0 * R * R;
        let G = T * (d.x * d.x + d.y * d.y);
        let H = 2.0 * T * (o.x * d.x + o.y * d.y);
        let I = T * (o.x * o.x + o.y * o.y);
        let J = d.length_squared();
        let K = 2.0 * o.to_dir().dot(d);
        let L = o.to_dir().length_squared() + R * R - S * S;

        let mut roots: Vec<f64> =
            solve_quartic_f64(J * J, 2.0 * J * K, 2.0 * J * L + K * K - G, 2.0 * K * L - H, L * L - I)
                .into_iter()
                .collect();

        roots.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let is = match roots.as_slice() {
            [] => Vec::with_capacity(0),
            [a] => {
                let i = self.intersection_for(&object_ray, *a);
                let i2 = i.clone();
                vec![Interval(i, i2)]
            }
            [a, b] => vec![Interval(
                self.intersection_for(&object_ray, *a),
                self.intersection_for(&object_ray, *b),
            )],
            [a, b, c] => {
                // Calculate the Intersections and determine the facing of their surface normals.
                // -1 means facing towards ray origin.
                // +1 means facing away from ray origin.
                let is: Vec<(Intersection, isize)> = [*a, *b, *c]
                    .into_iter()
                    .map(|&t| self.intersection_for(&object_ray, t))
                    .map(|i| (i, i.n.dot(d).signum() as isize))
                    .collect();

                match is.as_slice() {
                    // [] [ -- pair and single
                    [(ai, -1), (bi, 1), (ci, -1)] => vec![Interval(*ai, *bi), Interval(*ci, ci.clone())],
                    // ] [] -- single and pair
                    [(ai, 1), (bi, -1), (ci, 1)] => vec![Interval(*ai, ai.clone()), Interval(*bi, *ci)],
                    // wtf?!
                    _ => panic!("unhandled three roots solution for torus intersection: {:?}", [a, b, c]),
                }
            }
            [a, b, c, d] => vec![
                Interval(
                    self.intersection_for(&object_ray, *a),
                    self.intersection_for(&object_ray, *b),
                ),
                Interval(
                    self.intersection_for(&object_ray, *c),
                    self.intersection_for(&object_ray, *d),
                ),
            ],
            _ => panic!("unexpected number of quartic roots: {:?}", roots),
        };

        is.into_iter().map(|i| i.to_world(ray, &object_ray, &self.tx)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use direction::*;
    use point::*;
    use system::Ray;
    use test_utils::*;

    #[test]
    pub fn front_orthogonal_intersection() {
        let t = Torus::new(1.0, 0.1);
        let r = Ray::primary(Point::new(0.0, 1.0, 1.0), Direction::new(0.0, 0.0, -1.0), 0);
        let is: Vec<Intersection> = t
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = is.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = is.iter().map(|i| i.n).collect();
        assert_approx_eq!(distances, vec![0.9, 1.1]);
        assert_approx_eq!(
            normals,
            vec![Direction::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0)]
        );
    }

    #[test]
    pub fn front_oblique_intersection() {
        let t = Torus::new(1.0, 0.1);
        let o = Point::new(0.0, 0.0, 1.0);
        let d = (Point::new(0.0, 1.0, 0.0) - o).normalize();
        let r = Ray::primary(o, d, 0);
        let is: Vec<Intersection> = t
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = is.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = is.iter().map(|i| i.n).collect();
        assert_approx_eq!(distances, vec![0.9, 1.1]);
        assert_approx_eq!(
            normals,
            vec![Direction::new(0.0, 0.0, 1.0), Direction::new(0.0, 0.0, -1.0)]
        );
    }

    #[test]
    pub fn top_lateral_intersection() {
        let t = Torus::new(1.0, 0.1);
        let r = Ray::primary(Point::new(0.0, 2.0, 0.0), Direction::new(0.0, -1.0, 0.0), 0);
        let is: Vec<Intersection> = t
            .intersection_intervals(&r)
            .into_iter()
            .flat_map(|Interval(a, b)| vec![a, b])
            .collect();
        let distances: Vec<f64> = is.iter().map(|i| i.t).collect();
        let normals: Vec<Direction> = is.iter().map(|i| i.n).collect();
        assert_approx_eq!(distances, vec![0.9, 1.1, 2.9, 3.1]);
        assert_approx_eq!(
            normals,
            vec![
                Direction::new(0.0, 1.0, 0.0),
                Direction::new(0.0, -1.0, 0.0),
                Direction::new(0.0, 1.0, 0.0),
                Direction::new(0.0, -1.0, 0.0),
            ]
        );
    }
}
