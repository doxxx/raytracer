use std::cmp;
use std::f64;
use std::sync::Arc;
use std::sync::Mutex;

use rand::prelude::*;
use rayon::prelude::*;

use crate::color::Color;
use crate::direction::Direction;
use crate::matrix::Matrix44f;
use crate::object::Object;
use crate::object::Transformation;
use crate::point::Point;
use crate::sdl::Scene;
use crate::vector::Vector2f;

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

    pub fn trace<'object, 'ray>(
        &'ray self,
        objects: &'object [Object],
        max_distance: f64,
    ) -> Option<RayHit<'ray, 'object>> {
        objects
            .into_iter()
            .flat_map(|o| o.intersect(self).map(|i| (o, i))) // intersect with each object
            .filter(|(_, i)| i.t < max_distance) // exclude intersections beyond max_distance
            .min_by(|(_, a), (_, b)| a.t.partial_cmp(&b.t).unwrap()) // find the nearest
            .map(|(o, i)| RayHit::new(self, o, i)) // create RayHit
    }

    pub fn hit_color(&self, context: &RenderContext, hit: &RayHit) -> Color {
        let e = hit.object.material.emit(context, hit);
        let sr = hit.object.material.scatter(context, hit);
        let s = sr.map(|s| s.attenuation * Ray::primary(s.origin, s.direction, self.depth + 1).cast(context));
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
        RayHit {
            object,
            incident,
            t: i.t,
            n: i.n,
            uv: i.uv,
        }
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
    pub sqrt_spp: u32,
    pub recip_sqrt_spp: f64,
}

pub trait RenderProgress {
    fn render_started(&mut self, options: &Options);
    fn sample_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>);
    fn render_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>);
}

fn alloc_render_buf(width: u32, height: u32) -> Vec<Vec<Color>> {
    let mut renderbuf: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    let mut renderbuf_row: Vec<Color> = Vec::with_capacity(width as usize);
    renderbuf_row.resize(width as usize, Color::black());
    renderbuf.resize(height as usize, renderbuf_row);
    renderbuf
}

fn get_stratified_ray(context: &RenderContext, x: u32, y: u32, s_i: u32, s_j: u32) -> Ray {
    let mut rng = rand::rng();
    let s_x = ((s_i as f64 + rng.random::<f64>()) * context.recip_sqrt_spp) - 0.5;
    let s_y = ((s_j as f64 + rng.random::<f64>()) * context.recip_sqrt_spp) - 0.5;
    context.scene.camera.pixel_ray(x as f64 + s_x, y as f64 + s_y)
}

fn render_sample(context: &RenderContext, buf: &mut Vec<Vec<Color>>, s_i: u32, s_j: u32) {
    buf.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            let x = x as u32;
            let y = y as u32;
            let ray = get_stratified_ray(context, x, y, s_i, s_j);
            *pixel = ray.cast(&context);
        });
    });
}

fn combine_renderbuf(dest: &mut Vec<Vec<Color>>, src: &Vec<Vec<Color>>) {
    dest.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            pixel.add(&src[y][x]);
        });
    });
}

pub fn render<T>(options: Options, scene: Scene, progress: &mut Arc<Mutex<T>>)
where
    T: RenderProgress + Send,
{
    {
        let mut progress_guard = progress.lock().unwrap();
        progress_guard.render_started(&options);
    }

    let render_buf = Arc::new(Mutex::new(alloc_render_buf(options.width, options.height)));
    let context = Arc::new(RenderContext {
        options,
        scene,
        sqrt_spp: (options.samples as f64).sqrt() as u32,
        recip_sqrt_spp: (options.samples as f64).sqrt().recip(),
    });

    {
        let render_buf = render_buf.clone();
        let progress = progress.clone();

        let strat_coords: Vec<(u32, u32)> = (0..context.sqrt_spp)
            .flat_map(|i| (0..context.sqrt_spp).map(move |j| (i, j)))
            .collect();

        strat_coords.into_par_iter().for_each(move |(s_i, s_j)| {
            let mut sample_buf = alloc_render_buf(options.width, options.height);

            render_sample(&context, &mut sample_buf, s_i, s_j);

            {
                let mut render_buf_guard = render_buf.lock().unwrap();
                combine_renderbuf(&mut render_buf_guard, &sample_buf);
                let mut progress_guard = progress.lock().unwrap();
                progress_guard.sample_finished(&options, &render_buf_guard);
            }
        });
    }

    {
        let render_buf_guard = render_buf.lock().unwrap();
        let mut progress_guard = progress.lock().unwrap();
        progress_guard.render_finished(&options, &render_buf_guard);
    }
}
