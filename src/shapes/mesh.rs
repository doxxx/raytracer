use matrix::Matrix44f;
use object::Transformation;
use std::f64;

use direction::{Direction, Dot};
use point::Point;
use shapes::bounding_box::BoundingBox;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray, Transformable};
use vector::Vector2f;

pub struct Mesh {
    vertices: Vec<Point>,
    normals: Vec<Direction>,
    triangles: Vec<MeshTriangle>,
    bounding_box: BoundingBox,
    smooth_shading: bool,
    tx: Transformation,
}

pub struct MeshTriangle {
    pub vertex_indices: [usize; 3],
    pub normal_indices: [usize; 3],
}

impl Mesh {
    pub fn new(
        vertices: Vec<Point>,
        normals: Vec<Direction>,
        triangles: Vec<MeshTriangle>,
        smooth_shading: bool,
    ) -> Mesh {
        let mut min = Point::zero();
        let mut max = Point::zero();

        for v in &vertices {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);
            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        Mesh {
            vertices,
            normals,
            triangles,
            bounding_box: BoundingBox::new(min, max),
            smooth_shading,
            tx: Transformation::new(),
        }
    }

    fn intersect_triangle(&self, ray: &Ray, triangle: &MeshTriangle) -> Option<Intersection> {
        let v0 = self.vertices[triangle.vertex_indices[0]];
        let v1 = self.vertices[triangle.vertex_indices[1]];
        let v2 = self.vertices[triangle.vertex_indices[2]];
        let n0 = self.normals[triangle.normal_indices[0]];
        let n1 = self.normals[triangle.normal_indices[1]];
        let n2 = self.normals[triangle.normal_indices[2]];

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = ray.direction.cross(v0v2);
        let det = v0v1.dot(pvec);

        if det < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - v0;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = v0v2.dot(qvec) * inv_det;
        if t < 0.0 {
            return None;
        }

        let n = if self.smooth_shading {
            ((1.0 - u - v) * n0 + u * n1 + v * n2).normalize()
        } else {
            (n0 + n1 + n2).normalize()
        };

        Some(Intersection {
            t,
            n,
            uv: Vector2f(u, v),
        })
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersect(ray) {
            return None;
        }

        self.triangles
            .iter()
            .filter_map(|triangle| self.intersect_triangle(ray, triangle))
            .nth(0)
    }
}

impl Shape for Mesh {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn transformation(&self) -> &Transformation {
        &self.tx
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        // TODO: find all triangle intersections
        // TODO: if even then assume closed shape and pair intersections as intervals
        // TODO: or maybe require a flag to indicate whether mesh is closed
        self.intersect(ray)
            .map(|i| vec![Interval(i, i.clone())])
            .unwrap_or(Vec::with_capacity(0))
    }
}
