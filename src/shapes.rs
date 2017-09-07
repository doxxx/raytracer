use std::f64;
use std::mem;

use direction::{Dot, Direction};
use matrix::Matrix44f;
use point::Point;
use system::{Intersectable, Intersection, Ray};
use vector::Vector2f;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Mesh(Mesh),
    Composite(Composite),
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            &Shape::Sphere(ref s) => s.intersect(ray),
            &Shape::Plane(ref s) => s.intersect(ray),
            &Shape::Mesh(ref s) => s.intersect(ray),
            &Shape::Composite(ref s) => s.intersect(ray),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    bounds: [Point; 2],
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox { bounds: [min, max] }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let mut tmax = (self.bounds[1 - ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;
        let tymax = (self.bounds[1 - ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;
        let tzmax = (self.bounds[1 - ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        // if tzmin > tmin {
        //     tmin = tzmin;
        // }
        // if tzmax < tmax {
        //     tmax = tzmax;
        // }

        return true;
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {
            center: Point::zero(),
            radius_squared: radius.powi(2),
        }
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return None;
    } else if discr == 0.0 {
        let x = -0.5 * b / a;
        return Some((x, x));
    } else {
        let q = if b > 0.0 {
            -0.5 * (b + discr.sqrt())
        } else {
            -0.5 * (b - discr.sqrt())
        };
        Some((q / a, c / q))
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(l);
        let c = l.dot(l) - self.radius_squared;
        if let Some((mut t0, mut t1)) = solve_quadratic(a, b, c) {
            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }
            if t0 < 0.0 {
                t0 = t1;
                if t0 < 0.0 {
                    return None;
                }
            }

            let p = ray.origin + ray.direction * t0;
            let n = (p - self.center).normalize();
            let u = (1.0 - n.z.atan2(n.x) / f64::consts::PI) * 0.5;
            let v = n.y.acos() / f64::consts::PI;

            Some(Intersection {
                t: t0,
                n: n,
                uv: Vector2f(u, v),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    point: Point,
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
            point: Point::zero(),
            normal: normal,
            u: u,
            v: v,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }
        let w = ray.origin - self.point;
        let t = -self.normal.dot(w) / denom;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;
        let uv = Vector2f(self.u.dot(p - self.point), self.v.dot(p - self.point));
        Some(Intersection {
            t: t,
            n: self.normal,
            uv: uv,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Point>,
    pub normals: Vec<Direction>,
    pub triangles: Vec<MeshTriangle>,
    pub bounding_box: BoundingBox,
    pub smooth_shading: bool,
}

#[derive(Debug, Clone)]
pub struct MeshTriangle {
    pub vertex_indices: [usize; 3],
    pub normal_indices: [usize; 3],
}

impl Mesh {
    pub fn new(vertices: Vec<Point>, normals: Vec<Direction>, triangles: Vec<MeshTriangle>, smooth_shading: bool) -> Mesh {
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
            vertices: vertices,
            normals: normals,
            triangles: triangles,
            bounding_box: BoundingBox::new(min, max),
            smooth_shading: smooth_shading,
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
            t: t,
            n: n,
            uv: Vector2f(u, v),
        })
    }
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounding_box.intersect(ray) {
            return None;
        }

        let mut nearest = None;

        for triangle in &self.triangles {
            let i = self.intersect_triangle(ray, triangle);
            if let Some(i) = i {
                nearest = match nearest {
                    None => Some(i),
                    Some(n) => if i.t < n.t { Some(i) } else { Some(n) },
                }
            }
        }

        nearest
    }
}

#[derive(Debug, Clone)]
pub struct Composite {
    shapes: Vec<Shape>,
}

impl Composite {
    pub fn new(shapes: Vec<Shape>) -> Composite {
        Composite {
            shapes: shapes,
        }
    }
}

impl Intersectable for Composite {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes.iter()
            .map(|s| s.intersect(ray))
            .find(|i| i.is_some())
            .unwrap_or_default()
    }
}
