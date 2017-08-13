use std::f64;
use std::mem;

use color::Color;
use direction::{Dot,Direction};
use lights::{LightSource, Light};
use material::Material;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use shapes::{Shape,Intersectable};
use vector::{Vector2f};

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub num_threads: usize,
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub bias: f64,
    pub max_depth: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub width: f64,
    pub height: f64,
    aspect_ratio: f64,
    fov_factor: f64,
    camera_to_world: Matrix44f,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: f64) -> Camera {
        Camera {
            width: width as f64,
            height: height as f64,
            aspect_ratio: width as f64 / height as f64,
            fov_factor: (fov * 0.5).to_radians().tan(),
            camera_to_world: Matrix44f::identity(),
        }
    }

    pub fn transform(&mut self, m: Matrix44f) {
        self.camera_to_world = self.camera_to_world * m;
    }

    fn pixel_ray(&self, x: u32, y: u32) -> Ray {
        let ndcx = (x as f64 + 0.5) / self.width;
        let ndcy = (y as f64 + 0.5) / self.height;
        let cx = (2.0 * ndcx - 1.0) * self.fov_factor * self.aspect_ratio;
        let cy = (1.0 - 2f64 * ndcy) * self.fov_factor;
        let origin = Point::zero() * self.camera_to_world;
        let dir_point = Point::new(cx, cy, -1.0) * self.camera_to_world;
        Ray::primary(origin, (dir_point - origin).normalize())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayKind {
    Normal,
    Shadow,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub kind: RayKind,
    pub origin: Point,
    pub direction: Direction,
    pub inverse_direction: Direction,
    pub sign: [usize; 3],
}

impl Ray {
    pub fn primary(origin: Point, direction: Direction) -> Ray {
        Ray::new(RayKind::Normal, origin, direction)
    }

    pub fn shadow(origin: Point, direction: Direction) -> Ray {
        Ray::new(RayKind::Shadow, origin, direction)
    }

    fn new(kind: RayKind, origin: Point, direction: Direction) -> Ray {
        let inverse_direction = 1.0 / direction;
        Ray {
            kind: kind,
            origin: origin,
            direction: direction,
            inverse_direction: inverse_direction,
            sign: inverse_direction.sign(),
        }
    }
}

#[derive(Debug)]
struct RayHit<'a> {
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
    pub n: Direction,
    pub uv: Vector2f,
}

fn clamp(lo: f64, hi: f64, val: f64) -> f64 {
    lo.max(hi.min(val))
}

fn reflect(incident: Direction, normal: Direction) -> Direction {
    incident - normal * 2.0 * incident.dot(normal)
}

fn refract(incident: Direction, normal: Direction, ior: f64) -> Direction {
    let mut cos_i = clamp(-1.0, 1.0, incident.dot(normal));
    let mut eta_i = 1.0;
    let mut eta_t = ior;
    let mut n = normal;
    if cos_i < 0.0 {
        cos_i = -cos_i;
    } else {
        mem::swap(&mut eta_i, &mut eta_t);
        n = -normal;
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
    if k < 0.0 {
        Direction::zero()
    } else {
        incident * eta + n * (eta * cos_i - k.sqrt())
    }
}

//// incident, normal, index of reflection -> reflection
fn fresnel(incident: Direction, normal: Direction, ior: f64) -> f64 {
    let mut cos_i = clamp(-1.0, 1.0, incident.dot(normal));
    let mut eta_i = 1.0;
    let mut eta_t = ior;
    if cos_i > 0.0 {
        mem::swap(&mut eta_i, &mut eta_t);
    }
    let sin_t = eta_i / eta_t * (1.0 - cos_i * cos_i).max(0.0).sqrt();

    if sin_t >= 1.0 {
        // total internal reflection
        1.0
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        cos_i = cos_i.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        (r_s * r_s + r_p * r_p) / 2.0
    }
}

fn trace(ray: Ray, objects: &[Object], max_distance: f64) -> Option<RayHit> {
    let mut nearest_distance = max_distance;
    let mut nearest: Option<RayHit> = None;

    for object in objects {
        let intersection = match &object.shape {
            &Shape::Sphere(ref s) => s.intersect(ray),
            &Shape::Plane(ref s) => s.intersect(ray),
            &Shape::Triangle(ref s) => s.intersect(ray),
            &Shape::Mesh(ref s) => s.intersect(ray),
        };
        if let Some(intersection) = intersection {
            if intersection.t < nearest_distance {
                match (ray.kind, object.material) {
                    (RayKind::Shadow, Material::ReflectiveAndRefractive(_)) => {}
                    _ => {
                        nearest_distance = intersection.t;
                        nearest = Some(RayHit::new(&object, intersection));
                    }
                }
            }
        }
    }

    nearest
}

fn cast_ray(options: &Options, objects: &[Object], lights: &[Light], ray: Ray, depth: u16) -> Color {
    if depth > options.max_depth {
        return options.background_color;
    }

    let maybe_hit = trace(ray, &objects, f64::MAX);

    if let Some(hit) = maybe_hit {
        let hit_distance = hit.i.t;
        let hit_point = ray.origin + ray.direction * hit_distance;
        let hit_normal = hit.i.n;

        let mut hit_color = Color::zero();

        match hit.object.material {
            Material::Diffuse(color) => {
                for light in lights {
                    let (dir, intensity, distance) = match light {
                        &Light::Distant(ref light) => light.illuminate(hit_point),
                        &Light::Point(ref light) => light.illuminate(hit_point),
                    };
                    let shadow_ray = Ray::shadow(hit_point + hit_normal * options.bias, -dir);
                    let maybe_shadow_hit = trace(shadow_ray, objects, distance);
                    if maybe_shadow_hit.is_none() {
                        let albedo = hit.object.albedo;
                        let dot = hit_normal.dot(-dir);
                        if dot > 0.0 {
                            hit_color += color * albedo * intensity * dot;
                        }
                    }
                }
            }
            Material::Reflective => {
                let reflection_ray = Ray::primary(
                    hit_point + hit_normal * options.bias,
                    reflect(ray.direction, hit_normal).normalize(),
                );
                let reflection_color = cast_ray(options, objects, lights, reflection_ray, depth + 1);
                hit_color += reflection_color * 0.8;
            }
            Material::ReflectiveAndRefractive(ior) => {
                let mut refraction_color = Color::zero();
                let kr = fresnel(ray.direction, hit_normal, ior);
                let outside = ray.direction.dot(hit_normal) < 0.0;
                let bias = hit_normal * options.bias;
                if kr < 1.0 {
                    let refraction_ray = Ray::primary(
                        if outside {
                            hit_point - bias
                        } else {
                            hit_point + bias
                        },
                        refract(ray.direction, hit_normal, ior).normalize(),
                    );
                    refraction_color = cast_ray(options, objects, lights, refraction_ray, depth + 1);
                }
                let reflection_ray = Ray::primary(
                    if outside {
                        hit_point + bias
                    } else {
                        hit_point - bias
                    },
                    reflect(ray.direction, hit_normal).normalize(),
                );
                let reflection_color = cast_ray(options, objects, lights, reflection_ray, depth + 1);
                hit_color += reflection_color * kr + refraction_color * (1.0 - kr);
            }
        }

        hit_color
    } else {
        options.background_color
    }
}

pub fn calculate_pixel_color(
    options: &Options,
    camera: &Camera,
    objects: &[Object],
    lights: &[Light],
    x: u32,
    y: u32,
) -> Color {
    cast_ray(options, objects, lights, camera.pixel_ray(x, y), 0)
}
