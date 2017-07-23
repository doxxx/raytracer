use std::f64;

use object::Object;
use vector::Vector3f;

#[derive(Debug)]
pub struct Camera {
    width: f64,
    height: f64,
    aspect_ratio: f64,
    fov_factor: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f64) -> Camera {
        Camera {
            width: width as f64,
            height: height as f64,
            aspect_ratio: width as f64 / height as f64,
            fov_factor: (fov * 0.5).to_radians().tan(),
        }
    }

    pub fn pixel_ray(&self, x: u32, y: u32) -> Ray {
        let ndcx = (x as f64 + 0.5) / self.width;
        let ndcy = (y as f64 + 0.5) / self.height;
        let cx = (2.0 * ndcx - 1.0) * self.fov_factor * self.aspect_ratio;
        let cy = (1.0 - 2f64 * ndcy) * self.fov_factor;
        Ray {
            origin: Vector3f::zero(),
            direction: Vector3f(cx, cy, -1.0).normalize(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn project(&self, t: f64) -> Vector3f {
        self.origin + self.direction * t
    }
}

pub fn cast_ray(ray: Ray, objects: &[Object]) -> Option<(&Object, f64)> {
    objects
        .iter()
        .flat_map(|o| {
            o.shape.intersect(ray.origin, ray.direction).map(|i| (o, i))
        })
        .min_by(|&(_, i1), &(_, i2)| i1.partial_cmp(&i2).unwrap())
}
