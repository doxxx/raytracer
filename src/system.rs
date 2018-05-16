use std::f64;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;
use std::io::Stdout;
use std::ops::Deref;

use image;
use pbr::ProgressBar;
use time;
use rand;
use rand::Rng;

use color::Color;
use direction::Direction;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use sdl::Scene;
use vector::Vector2f;

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub num_threads: usize,
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub bias: f64,
    pub max_depth: u16,
    pub samples: u16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    location: Point,
    fov_factor: f64,
    camera_to_world: Matrix44f,
}

impl Camera {
    pub fn new(location: Point, fov: f64) -> Camera {
        Camera {
            location,
            fov_factor: (fov * 0.5).to_radians().tan(),
            camera_to_world: Matrix44f::identity(),
        }
    }

    pub fn look_at(&self, p: Point) -> Camera {
        let forward = (self.location - p).normalize();
        let right = Direction::new(0.0, 1.0, 0.0).normalize().cross(forward);
        let up = forward.cross(right);
        Camera {
            location: self.location,
            fov_factor: self.fov_factor,
            camera_to_world: Matrix44f(
                [
                    [right.x, right.y, right.z, 0.0],
                    [up.x, up.y, up.z, 0.0],
                    [forward.x, forward.y, forward.z, 0.0],
                    [self.location.x, self.location.y, self.location.z, 1.0],
                ]
            ),
        }
    }

    pub fn transform(&self, m: Matrix44f) -> Camera {
        Camera {
            location: self.location * m,
            fov_factor: self.fov_factor,
            camera_to_world: self.camera_to_world * m,
        }
    }

    fn pixel_ray(&self, width: u32, height: u32, x: f64, y: f64) -> Ray {
        let w = width as f64;
        let h = height as f64;
        let aspect_ratio = w / h;
        let ndcx = x / w;
        let ndcy = y / h;
        let cx = (2.0 * ndcx - 1.0) * self.fov_factor * aspect_ratio;
        let cy = (1.0 - 2.0 * ndcy) * self.fov_factor;
        let origin = Point::zero() * self.camera_to_world;
        let dir_point = Point::new(cx, cy, -1.0) * self.camera_to_world;
        Ray::primary(origin, (dir_point - origin).normalize(), 0)
    }

    fn pixel_rays(&self, count: u16, width: u32, height: u32, x: u32, y: u32) -> Vec<Ray> {
        let mut rng = rand::thread_rng();
        let mut rays = Vec::new();
        for _ in 0..count {
            rays.push(self.pixel_ray(width, height, x as f64 + rng.next_f64(), y as f64 + rng.next_f64()));
        }
        rays
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


    pub fn cast(&self, context: &RenderContext) -> Color {
        if self.depth > context.options.max_depth {
            return context.options.background_color;
        }

        match self.trace(&context.scene.objects, f64::MAX) {
            None => context.options.background_color,
            Some(hit) => {
                let si = SurfaceInfo {
                    incident: *self,
                    point: hit.i.point(self),
                    n: hit.i.n.clone(),
                    uv: hit.i.uv.clone(),
                };

                hit.object.material.color(context, &si)
            }
        }
    }

    pub fn trace<'a>(&self, objects: &'a [Object], max_distance: f64) -> Option<RayHit<'a>> {
        let mut nearest_distance = max_distance;
        let mut nearest: Option<RayHit> = None;

        for object in objects {
            let intersection = object.intersect(self);
            if let Some(intersection) = intersection {
                if intersection.t < nearest_distance {
                    // HACK: transparent objects don't cast shadows
                    if self.kind == RayKind::Shadow && object.material.has_transparency() { continue }

                    nearest_distance = intersection.t;
                    nearest = Some(RayHit::new(&object, intersection));
                }
            }
        }

        nearest
    }
}

impl Transformable for Ray {
    fn transform(self, m: Matrix44f) -> Self {
        Ray::new(self.kind, self.origin * m, self.direction * m.inverse().transposed(), self.depth)
    }
}

pub struct RayHit<'a> {
    pub object: &'a Object,
    pub i: Intersection,
}

impl<'a> RayHit<'a> {
    pub fn new(object: &Object, i: Intersection) -> RayHit {
        RayHit { object, i }
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub n: Direction,
    pub uv: Vector2f,
}

impl Intersection {
    pub fn point(&self, ray: &Ray) -> Point {
        ray.origin + ray.direction * self.t
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Transformable {
    fn transform(self, m: Matrix44f) -> Self;
}

pub struct RenderContext {
    pub options: Options,
    pub scene: Scene,
}

pub struct SurfaceInfo {
    pub incident: Ray,
    pub point: Point,
    pub n: Direction,
    pub uv: Vector2f,
}

fn color_to_rgb(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn color_at_pixel(context: &RenderContext, x: u32, y: u32) -> Color {
    let rays = context.scene.camera.pixel_rays(context.options.samples, context.options.width, context.options.height, x, y);
    let mut color = Color::black();
    for ray in rays.iter() {
        color += ray.cast(&context);
    }
    color / (rays.len() as f64)
}

pub fn render(
    options: Options,
    scene: Scene,
) -> image::RgbImage {
    let width = options.width;
    let height = options.height;

    let context = RenderContext {
        options,
        scene,
    };

    println!("Rendering {}x{}, {} samples per pixel.", width, height, options.samples);

    let start_time = time::now();
    let steady_start_time = time::SteadyTime::now();

    println!("Started rendering at: {}", start_time.rfc822());

    // start progress bar update task
    let mut pb: ProgressBar<Stdout> = ProgressBar::new(height as u64);
    pb.message(&format!("Rendering (x{}): ", options.num_threads));

    let mut results: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    results.resize(height as usize, Vec::new());
    let results: Arc<Mutex<Vec<Vec<Color>>>> = Arc::new(Mutex::new(results));

    if options.num_threads > 1 {
        let context = Arc::new(context);
        let rows: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..height).collect()));
        let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

        // spawn threads for rendering rows
        // each thread sends the row index when it finishes rendering
        for _ in 0..options.num_threads {
            let context = context.clone();
            let rows = rows.clone();
            let results = results.clone();
            let tx = tx.clone();
            spawn(move || {
                loop {
                    let y = { rows.lock().unwrap().pop() };
                    match y {
                        Some(y) => {
                            let row = (0..width).map(|x| {
                                let context = context.deref();
                                color_at_pixel(&context, x, y)
                            }).collect();
                            let mut results = results.lock().unwrap();
                            results[y as usize] = row;
                            let _ = tx.send(y);
                        }
                        None => break
                    }
                }
            });
        }

        // wait for all the rows to be rendered,
        // updating progress as each row is finished
        for _ in 0..pb.total {
            let _ = rx.recv();
            pb.inc();
        }
    } else {
        for y in 0..height {
            let row = (0..width).map(|x| color_at_pixel(&context, x, y)).collect();
            let mut results = results.lock().unwrap();
            results[y as usize] = row;
        }
    }

    let end_time = time::now();
    let elapsed = time::SteadyTime::now() - steady_start_time;

    pb.finish_println(&format!("Finished rendering at: {}\n", end_time.rfc822()));

    println!("Elapsed time: {}", elapsed);

    let results = results.lock().unwrap();
    let mut imgbuf = image::RgbImage::new(options.width, options.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color_to_rgb(results[y as usize][x as usize]);
    }

    imgbuf
}
