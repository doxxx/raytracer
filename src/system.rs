use std::cmp;
use std::f64;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread::spawn;

use rand;
use rand::Rng;

use color::*;
use direction::*;
use materials::*;
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

    fn random_pixel_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = rand::thread_rng();
        self.pixel_ray(x as f64 + rng.gen::<f64>(), y as f64 + rng.gen::<f64>())
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
                let emitted = hit.object.material.emit(context, &hit);
                match hit.object.material.interact(context, &hit) {
                    MaterialInteraction::Absorbed => emitted,
                    MaterialInteraction::Scattered{albedo, dir, pdf} => {
                        if self.depth < context.options.max_depth {
                            let mut rng = rand::thread_rng();
                            let lights: Vec<&Object> = context.scene.objects.iter()
                                .filter(|o| o.material.kind() == MaterialKind::Emitting)
                                .collect();
                            let on_lights: Vec<Point> = lights.iter()
                                .map(|l| l.position() + Point::new(rng.gen::<f64>() * 130.0 - 65.0, 0.0, rng.gen::<f64>() * 105.0 - 52.5))
                                .collect();
                            let to_lights: Vec<Direction> = on_lights.into_iter().map(|l| (l - hit.p)).collect();
                            let light_distances: Vec<f64> = to_lights.iter().map(|d| d.length_squared()).collect();
                            let to_lights: Vec<Direction> = to_lights.into_iter().map(|d| d.normalize()).filter(|d| d.dot(hit.n) >= 0.0).collect();
                            if to_lights.len() == 0 {
                                emitted
                            } else {
                                let light_areas: Vec<f64> = lights.iter().map(|l| 130.0 * 105.0).collect();
                                let light_cosines: Vec<f64> = to_lights.iter().map(|d| d.y).collect();
                                if light_cosines.iter().all(|c| *c < 0.000001) {
                                    emitted
                                } else {
                                    let pdfs: Vec<f64> = light_cosines.iter()
                                        .zip(light_areas).map(|(c,a)| c * a)
                                        .zip(light_distances).map(|(ca, d)| d / ca)
                                        .collect();
                                    let scattered_rays: Vec<Ray> = to_lights.iter()
                                        .map(|d| Ray::primary(hit.p, *d, self.depth+1))
                                        .collect();
                                    let scattering_pdfs: Vec<f64> = scattered_rays.iter()
                                        .map(|r| hit.object.material.scattering_pdf(context, &hit, r))
                                        .collect();
                                    let scattering_colors: Vec<Color> = scattered_rays.iter()
                                        .map(|r| r.cast(context))
                                        .collect();
                                    let scattering_colors: Vec<Color> = scattering_colors.into_iter()
                                        .zip(scattering_pdfs.into_iter())
                                        .map(|(c, spdf)| spdf * c)
                                        .zip(pdfs.into_iter())
                                        .map(|(c, pdf)| c / pdf)
                                        .collect();
                                    let num_scattering_colors = scattering_colors.len();

                                    let avg_scattering_color = scattering_colors.into_iter().sum::<Color>() / num_scattering_colors as f64;
                                    emitted + albedo * avg_scattering_color
                                }
                            }


                            // let scattering_pdf = hit.object.material.scattering_pdf(context, &hit, &dir);
                            // emitted + albedo * scattering_pdf * dir.cast(context) / pdf
                        } else {
                            emitted
                        }
                    }
                }
            }
        }
    }

    pub fn trace<'a>(&'a self, objects: &'a [Object], max_distance: f64) -> Option<RayHit<'a>> {
        let mut nearest_distance = max_distance;
        let mut nearest: Option<(&Object, Intersection)> = None;

        for object in objects {
            if let Some(i) = object.intersect(self) {
                if i.t < nearest_distance {
                    nearest_distance = i.t;
                    nearest = Some((&object, i));
                }
            }
        }

        nearest.map(|(o, i)| RayHit::new(self, o, i))
    }
}

impl Transformable for Ray {
    fn transform(self, m: Matrix44f) -> Self {
        Ray::new(self.kind, self.origin * m, self.direction * m.inverse().transposed(), self.depth)
    }
}

pub struct RayHit<'a> {
    pub incident: &'a Ray,
    pub object: &'a Object,
    pub t: f64,
    pub n: Direction,
    pub uv: Vector2f,
    pub p: Point,
}


impl<'a> RayHit<'a> {
    pub fn new(ray: &'a Ray, object: &'a Object, i: Intersection) -> RayHit<'a> {
        RayHit { 
            incident: ray, 
            object, 
            t: i.t, 
            n: i.n, 
            uv: i.uv, 
            p: ray.origin + ray.direction * i.t
        }
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
    fn transform(self, m: Matrix44f) -> Self;
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
                loop {
                    let y = rows.lock().unwrap().pop();
                    if let Some(y) = y {
                        let row: Vec<Color> = (0..width).map(|x| color_at_pixel(&context, x, y)).collect();

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
