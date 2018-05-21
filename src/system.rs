use std::f64;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread::spawn;

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
    pub bias: f64,
    pub max_depth: u16,
    pub samples: u16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    width: f64,
    height: f64,
    origin: Point,
    fov_factor: f64,
    camera_to_world: Matrix44f,
}

impl Camera {
    pub fn new(width: f64, height: f64, origin: Point, fov: f64) -> Camera {
        Camera {
            width,
            height,
            origin,
            fov_factor: (fov * 0.5).to_radians().tan(),
            camera_to_world: Matrix44f::identity(),
        }
    }

    pub fn look_at(&self, p: Point) -> Camera {
        let forward = (self.origin - p).normalize();
        let right = Direction::new(0.0, 1.0, 0.0).normalize().cross(forward);
        let up = forward.cross(right);
        Camera {
            width: self.width,
            height: self.height,
            origin: self.origin,
            fov_factor: self.fov_factor,
            camera_to_world: Matrix44f(
                [
                    [right.x, right.y, right.z, 0.0],
                    [up.x, up.y, up.z, 0.0],
                    [forward.x, forward.y, forward.z, 0.0],
                    [self.origin.x, self.origin.y, self.origin.z, 1.0],
                ]
            ),
        }
    }

    pub fn transform(&self, m: Matrix44f) -> Camera {
        Camera {
            width: self.width,
            height: self.height,
            origin: self.origin * m,
            fov_factor: self.fov_factor,
            camera_to_world: self.camera_to_world * m,
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

    fn random_pixel_ray(&self, rng: &mut rand::Rng, x: u32, y: u32) -> Ray {
        self.pixel_ray(x as f64 + rng.next_f64(), y as f64 + rng.next_f64())
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
        match self.trace(&context.scene.objects, f64::MAX) {
            None => {
                context.scene.options.background_color
            },
            Some(hit) => {
                let si = SurfaceInfo {
                    incident: *self,
                    point: hit.i.point(self),
                    n: hit.i.n.clone(),
                    uv: hit.i.uv.clone(),
                };

                let i = hit.object.material.interact(context, &si);
                if self.depth < context.options.max_depth && !i.absorbed {
                    i.emittance + i.attenuation * i.scattered.cast(context)
                } else {
                    i.emittance
                }
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

pub trait RenderProgress {
    fn render_started(&mut self, options: &Options);
    fn sample_started(&mut self, options: &Options);
    fn row_finished(&mut self, options: &Options);
    fn sample_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16);
    fn render_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16);
}

fn color_at_pixel(context: &RenderContext, rng: &mut Rng, x: u32, y: u32) -> Color {
    context.scene.camera.random_pixel_ray(rng, x, y).cast(&context)
}

fn update_row(renderbuf: &mut Vec<Vec<Color>>, y: u32, new_row: &Vec<Color>) {
    let row = &mut renderbuf[y as usize];
    for i in 0..row.len() {
        row[i] = row[i] + new_row[i];
    }
}

pub fn render<T>(options: Options, scene: Scene, progress: &mut T)
where T: RenderProgress,
{
    progress.render_started(&options);

    // pre-allocate render buffer
    let width = options.width;
    let height = options.height;
    let mut renderbuf: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    let mut renderbuf_row: Vec<Color> = Vec::with_capacity(width as usize);
    renderbuf_row.resize(width as usize, Color::black());
    renderbuf.resize(height as usize, renderbuf_row);

    let context = Arc::new(RenderContext {
        options,
        scene,
    });
    let renderbuf = Arc::new(Mutex::new(renderbuf));

    for current_sample in 0..options.samples {
        progress.sample_started(&options);

        let rows: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..height).collect()));
        let (tx, rx) = mpsc::channel();

        // Spawn threads for rendering rows.
        // Each thread pulls a row index, renders the row, and sends the row index when finished.
        for _ in 0..options.num_threads {
            let context = context.clone();
            let rows = rows.clone();
            let renderbuf = renderbuf.clone();
            let tx = tx.clone();

            spawn(move || {
                let mut rng = rand::thread_rng();
                loop {
                    let y = rows.lock().unwrap().pop();
                    if let Some(y) = y {
                        let row: Vec<Color> = (0..width).map(|x| color_at_pixel(&context, &mut rng, x, y)).collect();

                        {
                            let mut renderbuf = renderbuf.lock().unwrap();
                            update_row(&mut renderbuf, y, &row);
                        }

                        let _ = tx.send(y);
                    } else {
                        break;
                    }
                }
            });
        }

        // Wait for all the rows to be rendered.
        for _ in 0..height {
            let _ = rx.recv();
            progress.row_finished(&options);
        }

        let renderbuf = renderbuf.lock().unwrap();
        progress.sample_finished(&options, &renderbuf, current_sample + 1);
    }

    let renderbuf = renderbuf.lock().unwrap();
    progress.render_finished(&options, &renderbuf, options.samples)
}
