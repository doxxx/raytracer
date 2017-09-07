use std::f64;
use std::mem;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;
use std::io::Stdout;

use image;
use pbr::ProgressBar;
use time;

use color::Color;
use direction::{Dot, Direction};
use lights::Light;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use scene::Scene;
use shader::Shader;
use vector::Vector2f;

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

#[derive(Debug, Copy, Clone)]
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


    pub fn cast(&self, context: &RenderContext, depth: u16) -> Color {
        if depth > context.options.max_depth {
            return context.options.background_color;
        }

        match self.trace(&context.scene.objects, f64::MAX) {
            None => context.options.background_color,
            Some(hit) => {
                let si = SurfaceInfo {
                    point: hit.i.point(self),
                    n: hit.i.n.clone(),
                    uv: hit.i.uv.clone(),
                };

                let mut color = Color::black();

                for &(factor, ref shader) in &hit.object.shaders {
                    color += factor * shader.shade_point(context, depth, self.direction, hit.object, &si);
                }

                color
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
                    if self.kind != RayKind::Shadow || !object.shaders.iter().any(|sa| {
                        match sa {
                            &(_, Shader::Transparency { .. }) => true,
                            _ => false,
                        }
                    }) {
                        nearest_distance = intersection.t;
                        nearest = Some(RayHit::new(&object, intersection));
                    }
                }
            }
        }

        nearest
    }
}

impl Transformable for Ray {
    fn transform(&self, m: Matrix44f) -> Self {
        Ray::new(self.kind, self.origin * m, self.direction * m.inverse().transposed())
    }
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
    fn transform(&self, m: Matrix44f) -> Self;
}

#[derive(Debug, Clone)]
pub struct RenderContext {
    pub options: Options,
    pub scene: Scene,
}

pub struct SurfaceInfo {
    pub point: Point,
    pub n: Direction,
    pub uv: Vector2f,
}

fn color_to_pixel(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

pub fn render(
    options: Options,
    scene: Scene,
) -> image::RgbImage {
    let mut imgbuf = image::RgbImage::new(options.width, options.height);
    let width = options.width;
    let height = options.height;
    let rows: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..height).collect()));
    let mut results: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    results.resize(height as usize, Vec::new());
    let results: Arc<Mutex<Vec<Vec<Color>>>> = Arc::new(Mutex::new(results));
    let context = RenderContext {
        options: options.clone(),
        scene: scene.clone(),
    };

    let start_time = time::now();
    let steady_start_time = time::SteadyTime::now();

    println!("Started rendering at: {}", start_time.rfc822());

    // start progress bar update task
    let mut pb: ProgressBar<Stdout> = ProgressBar::new(height as u64);
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    pb.message(&format!("Rendering (x{}): ", options.num_threads));

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
                        let row = (0..width)
                            .map(|x| {
                                context.scene.camera.pixel_ray(x, y).cast(&context, 0)
                            })
                            .collect();
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

    let end_time = time::now();
    let elapsed = time::SteadyTime::now() - steady_start_time;

    pb.finish_println(&format!("Finished rendering at: {}\n", end_time.rfc822()));

    println!("Elapsed time: {}", elapsed);

    let results = results.lock().unwrap();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color_to_pixel(results[y as usize][x as usize]);
    }

    imgbuf
}
