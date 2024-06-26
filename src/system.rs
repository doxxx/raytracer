use std::cmp;
use std::f64;
use std::sync::Arc;

use rand::prelude::*;
use rand::distributions::Uniform;
use rayon::prelude::*;

use color::Color;
use direction::Direction;
use matrix::Matrix44f;
use object::Object;
use object::Transformation;
use point::Point;
use sdl::Scene;
use vector::Vector2f;

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub num_threads: usize,
    pub width: u32,
    pub height: u32,
    pub bias: f64,
    pub max_depth: u16,
    pub samples: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    width: f64,
    height: f64,
    fov_factor: f64,
    camera_to_world: Matrix44f,
    uniform_dist: Uniform<f64>,
}

impl Camera {
    pub fn new(width: f64, height: f64, fov: f64, origin: Point, look_at: Point) -> Camera {
        let up = Direction::new(0.0, 1.0, 0.0);
        let zaxis = (origin - look_at).normalize();
        let xaxis = up.normalize().cross(zaxis);
        let yaxis = zaxis.cross(xaxis);
        let camera_to_world = Matrix44f([
            [xaxis.x, xaxis.y, xaxis.z, 0.0],
            [yaxis.x, yaxis.y, yaxis.z, 0.0],
            [zaxis.x, zaxis.y, zaxis.z, 0.0],
            [origin.x, origin.y, origin.z, 1.0],
        ]);

        Camera {
            width,
            height,
            fov_factor: (fov * 0.5).to_radians().tan(),
            camera_to_world,
            uniform_dist: Uniform::new(-0.5, 0.5),
        }
    }

    fn pixel_ray(&self, x: f64, y: f64) -> Ray {
        let aspect_ratio = self.width / self.height;
        let ndcx = x / self.width;
        let ndcy = y / self.height;
        let cx = (2.0 * ndcx - 1.0) * self.fov_factor * aspect_ratio;
        let cy = (1.0 - 2.0 * ndcy) * self.fov_factor;
        let origin = Point::zero() * self.camera_to_world;
        let dir_point = Point::new(cx, cy, -1.0) * self.camera_to_world;
        Ray::primary(origin, (dir_point - origin).normalize(), 0)
    }

    fn random_pixel_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let rand_x: f64 = 0.5 + rng.sample(self.uniform_dist);
        let rand_y: f64 = 0.5 + rng.sample(self.uniform_dist);
        self.pixel_ray(x as f64 + rand_x, y as f64 + rand_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayKind {
    Normal,
    Shadow,
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub kind: RayKind,
    pub origin: Point,
    pub direction: Direction,
    pub depth: u16,
    pub inverse_direction: Direction,
    pub sign: [usize; 3],
}

impl Ray {
    pub fn primary(origin: Point, direction: Direction, depth: u16) -> Ray {
        Ray::new(RayKind::Normal, origin, direction, depth)
    }

    pub fn shadow(origin: Point, direction: Direction, depth: u16) -> Ray {
        Ray::new(RayKind::Shadow, origin, direction, depth)
    }

    fn new(kind: RayKind, origin: Point, direction: Direction, depth: u16) -> Ray {
        let inverse_direction = 1.0 / direction;
        Ray {
            kind,
            origin,
            direction,
            depth,
            inverse_direction,
            sign: inverse_direction.sign(),
        }
    }

    pub fn to_object(&self, tx: &Transformation) -> Ray {
        let mut object_ray = self.clone();
        object_ray.transform(tx.world_to_object);
        object_ray
    }

    pub fn cast(&self, context: &RenderContext) -> Color {
        if self.depth >= context.options.max_depth {
            context.scene.options.background_color
        } else {
            self.trace(&context.scene.objects, f64::MAX)
                .map(|hit| self.hit_color(context, &hit))
                .unwrap_or(context.scene.options.background_color)
        }
    }

    pub fn trace<'object, 'ray>(&'ray self, objects: &'object [Object], max_distance: f64) -> Option<RayHit<'ray, 'object>> {
        objects.into_iter()
            .flat_map(|o| o.intersect(self).map(|i| (o, i)))         // intersect with each object
            .filter(|(_, i)| i.t < max_distance)                     // exclude intersections beyond max_distance
            .min_by(|(_, a), (_, b)| a.t.partial_cmp(&b.t).unwrap()) // find the nearest
            .map(|(o, i)| RayHit::new(self, o, i))                   // create RayHit
    }

    pub fn hit_color(&self, context: &RenderContext, hit: &RayHit) -> Color {
        let e = hit.object.material.emit(context, hit);
        let sr = hit.object.material.scatter(context, hit);
        let s = sr.map(|s| {
            s.attenuation * Ray::primary(s.origin, s.direction, self.depth + 1).cast(context)
        });
        let s = s.unwrap_or(context.scene.options.background_color);

        e + s
    }
}

impl Transformable for Ray {
    fn transform(&mut self, m: Matrix44f) {
        self.origin = self.origin * m;
        self.direction = (self.direction * m).normalize();
        self.inverse_direction = 1.0 / self.direction;
        self.sign = self.inverse_direction.sign();
    }
}

pub struct RayHit<'ray, 'object> {
    pub incident: &'ray Ray,
    pub object: &'object Object,
    pub t: f64,
    pub n: Direction,
    pub uv: Vector2f,
}

impl<'ray, 'object> RayHit<'ray, 'object> {
    pub fn new(incident: &'ray Ray, object: &'object Object, i: Intersection) -> RayHit<'ray, 'object> {
        RayHit { object, incident, t: i.t, n: i.n, uv: i.uv }
    }

    pub fn point(&self) -> Point {
        self.incident.origin + self.incident.direction * self.t
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub n: Direction,
    pub uv: Vector2f,
}

impl Intersection {
    pub fn point(&self, ray: &Ray) -> Point {
        ray.origin + ray.direction * self.t
    }

    pub fn to_world(&self, world_ray: &Ray, object_ray: &Ray, tx: &Transformation) -> Intersection {
        let object_hit_point = self.point(&object_ray);
        let world_hit_point = object_hit_point * tx.object_to_world;
        let tsign = self.t.signum();
        Intersection {
            t: tsign * (world_hit_point - world_ray.origin).length(),
            n: (self.n * tx.object_to_world.inverse().transpose()).normalize(),
            uv: self.uv,
        }
    }
}

impl cmp::PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(&mut self, m: Matrix44f);
}

pub struct RenderContext {
    pub options: Options,
    pub scene: Scene,
}

pub trait RenderProgress {
    fn render_started(&mut self, options: &Options);
    fn sample_started(&mut self, options: &Options);
    fn row_finished(&mut self, options: &Options);
    fn sample_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16);
    fn render_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16);
}

fn color_at_pixel(context: &RenderContext, x: u32, y: u32) -> Color {
    context.scene.camera.random_pixel_ray(x, y).cast(&context)
}

fn alloc_render_buf(width: u32, height: u32) -> Vec<Vec<Color>> {
    let mut renderbuf: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    let mut renderbuf_row: Vec<Color> = Vec::with_capacity(width as usize);
    renderbuf_row.resize(width as usize, Color::black());
    renderbuf.resize(height as usize, renderbuf_row);
    renderbuf
}

pub fn render<T>(options: Options, scene: Scene, progress: &mut T)
where T: RenderProgress,
{
    progress.render_started(&options);

    let mut renderbuf = alloc_render_buf(options.width, options.height);

    let context = Arc::new(RenderContext {
        options,
        scene,
    });

    for current_sample in 0..options.samples {
        progress.sample_started(&options);

        renderbuf.par_iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, pixel)| {
                pixel.add(&color_at_pixel(&context, x as u32, y as u32));
            });
        });

        progress.sample_finished(&options, &renderbuf, current_sample + 1);
    }

    progress.render_finished(&options, &renderbuf, options.samples)
}
