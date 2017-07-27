use std::f64;

use lights::Light;
use object::Object;
use vector::{Vector2f, Vector3f};

pub type Color = Vector3f;

#[derive(Debug)]
pub struct Options {
    pub bias: f64,
}

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
    pub fn new(origin: Vector3f, direction: Vector3f) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

pub fn cast_ray(ray: Ray, objects: &[Object], max_distance: f64) -> Option<RayHit> {
    let mut nearest_distance = max_distance;
    let mut nearest: Option<RayHit> = None;

    for object in objects {
        let maybe_intersection = object.shape.intersect(ray.origin, ray.direction);
        if let Some(intersection) = maybe_intersection {
            if intersection.t < nearest_distance {
                nearest_distance = intersection.t;
                nearest = Some(RayHit::new(&object, intersection));
            }
        }
    }

    nearest
}

#[derive(Debug)]
pub struct RayHit<'a> {
    pub object: &'a Object,
    pub i: Intersection,
}

impl<'a> RayHit<'a> {
    pub fn new(object: &Object, i: Intersection) -> RayHit {
        RayHit {
            object: object,
            i: i,
        }
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub n: Vector3f,
    pub uv: Vector2f,
}

pub fn calculate_pixel_color(
    options: &Options,
    camera: &Camera,
    objects: &Vec<Object>,
    lights: &Vec<Box<Light>>,
    x: u32,
    y: u32,
) -> Option<Color> {
    let ray = camera.pixel_ray(x, y);
    let maybe_hit = cast_ray(ray, &objects, f64::MAX);

    if let Some(hit) = maybe_hit {
        let hit_distance = hit.i.t;
        let hit_point = ray.origin + ray.direction * hit_distance;
        let hit_normal = hit.i.n;

        let mut hit_color = Vector3f::zero();

        for light in lights {
            let (dir, intensity, distance) = light.illuminate(hit_point);
            let shadow_ray = Ray::new(hit_point + hit_normal * options.bias, -dir);
            let maybe_shadow_hit = cast_ray(shadow_ray, objects, distance);
            if maybe_shadow_hit.is_none() {
                let albedo = hit.object.albedo;
                let dot = hit_normal.dot(-dir);
                if dot > 0.0 {
                    hit_color += albedo * intensity * dot;
                }
            }
        }

        Some(hit_color)
    } else {
        None
    }
}
