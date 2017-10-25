use std::f64;
use std::f64::consts::PI;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;
use std::io::Stdout;
use std::ops::Deref;

use image;
use pbr::ProgressBar;
use rand;
use time;

use color::Color;
use direction::{Direction, Dot};
use kdtree;
use lights::Light;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use sdl::Scene;
use shapes::bounding_box::BoundingBox;
use vector::Vector2f;

#[derive(Debug, Copy, Clone)]
pub struct Options {
    pub num_threads: usize,
    pub width: u32,
    pub height: u32,
    pub background_color: Color,
    pub bias: f64,
    pub max_depth: u16,
    pub antialiasing: bool,
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

    fn pixel_ray_bundle(&self, width: u32, height: u32, x: u32, y: u32) -> [Ray; 4] {
        [
            self.pixel_ray(width, height, x as f64 + 0.25, y as f64 + 0.25),
            self.pixel_ray(width, height, x as f64 + 0.75, y as f64 + 0.25),
            self.pixel_ray(width, height, x as f64 + 0.75, y as f64 + 0.75),
            self.pixel_ray(width, height, x as f64 + 0.25, y as f64 + 0.75),
        ]
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
    pub photon_map: Option<PhotonMap>,
}

pub struct SurfaceInfo {
    pub incident: Ray,
    pub point: Point,
    pub n: Direction,
    pub uv: Vector2f,
}

#[derive(Clone, Copy)]
pub struct PhotonData {
    pub power: Color,
    pub incident: Direction,
}

type PhotonMap = Box<kdtree::Tree<PhotonData>>;
type PhotonNode = kdtree::Data<PhotonData>;

fn color_to_rgb(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn color_at_pixel(context: &RenderContext, x: u32, y: u32) -> Color {
    if context.options.antialiasing {
        let rays = context.scene.camera.pixel_ray_bundle(context.options.width, context.options.height, x, y);
        let mut color = Color::black();
        for ray in rays.iter() {
            color += ray.cast(&context);
        }
        color / (rays.len() as f64)
    } else {
        context.scene.camera.pixel_ray(context.options.width, context.options.height, x as f64 + 0.5, y as f64 + 0.5).cast(&context)
    }
}

pub fn render(options: Options, scene: Scene) -> image::RgbImage {
    let photon_map = generate_photon_map(options, &scene);

    render_image(options, scene, photon_map)
}

fn generate_photon_map(options: Options, scene: &Scene) -> Option<PhotonMap> {
    let start_time = time::now();
    let steady_start_time = time::SteadyTime::now();

    let bb = scene.bounding_box();
    println!("Scene bounding box: {:?}", bb);

    let photons_per_light = 100000;
    let total_photon_count = scene.lights.len() * photons_per_light;

//    let mut pb: ProgressBar<Stdout> = ProgressBar::new(total_photon_count as u64);
//    pb.message(&format!("Photon mapping (x{}): ", 1)); // todo: options.num_threads

    println!("Generating {} photons...", total_photon_count);

    let mut photons: Vec<PhotonNode> = Vec::with_capacity(total_photon_count);

    let photon_power = 1.0 / photons_per_light as f64;

    for light in &scene.lights {
        let init = photons.len();
        let mut last_print = steady_start_time;
        loop {
            let p = bb.random_point();
            let ray = Ray::new(RayKind::Normal, light.origin(), (p - light.origin()).normalize(), 0);
            trace_photon(options, scene, light, ray, light.power() * photon_power, &mut photons);
            let now = time::SteadyTime::now();
            if (now - last_print).num_seconds() > 5 {
                println!("Generated {} photons.", photons.len());
                last_print = now;
            }
            if photons.len() - init >= photons_per_light {
                break;
            }
//            pb.inc();
        }
    }

    println!("Generated {} photons.", photons.len());

    let tree = kdtree::Tree::new(&photons);

    let end_time = time::now();
    let elapsed = time::SteadyTime::now() - steady_start_time;

//    pb.finish_println(&format!("Finished photon mapping at: {}\n", end_time.rfc822()));
    println!("Elapsed time: {}", elapsed);

    tree
}

const DIFFUSE_REFLECTION_PB: f64 = 0.5;
const SPECULAR_REFLECTION_PB: f64 = 0.2;

fn trace_photon(options: Options, scene: &Scene, light: &Box<Light>, ray: Ray, power: Color, photons: &mut Vec<PhotonNode>) {
    if ray.depth > options.max_depth {
        return;
    }

    if let Some(hit) = ray.trace(&scene.objects, f64::MAX) {
        let ip = hit.i.point(&ray);
        let si = SurfaceInfo {
            incident: ray,
            point: ip,
            n: hit.i.n,
            uv: hit.i.uv,
        };

//        let albedo = 0.18;
//        let lambertian = si.n.dot(-ray.direction).max(0.0) * albedo;
//        let irradiance = power * lambertian;

        let rr: f64 = rand::random();
        if rr < DIFFUSE_REFLECTION_PB {
            // diffuse reflection

            if ray.depth > 0 {
                photons.push(kdtree::Data::new(ip, PhotonData {
                    power,
                    incident: ray.direction,
                }));
            }

            let surface_color = hit.object.material.surface_color(&si);
            let reflected_power = surface_color * power;

            let reflected_dir = ray.direction.reflect(hit.i.n); // perfect reflection

            // random reflection (diffuse) -- loops until doesn't intersect current object
//            let mut reflected_dir: Direction = Direction::zero();
//            loop {
//                let bb = BoundingBox::new(Point::new(-10.0, -10.0, -10.0), Point::new(10.0, 10.0, 10.0));
//                let p = bb.random_point();
//
//                reflected_dir = (p - ip).normalize();
//
//                if hit.object.intersect(&Ray::new(RayKind::Normal, ip, reflected_dir, 0)).is_none() {
//                    break;
//                }
//            }

            let reflected_ray = Ray::new(
                RayKind::Normal,
                ip + options.bias * hit.i.n,
                reflected_dir,
                ray.depth + 1
            );
            trace_photon(options, scene, light, reflected_ray, reflected_power, photons)
        }
        else if rr < DIFFUSE_REFLECTION_PB + SPECULAR_REFLECTION_PB {
            // todo: specular reflection
        }
        else {
            // absorption

            // todo: store only if surface is diffuse
            if ray.depth > 0 {
                photons.push(kdtree::Data::new(ip, PhotonData {
                    power,
                    incident: ray.direction,
                }));
            }
        }
    }
}

fn render_image(options: Options, scene: Scene, photon_map: Option<PhotonMap>) -> image::RgbImage {
    let start_time = time::now();
    let steady_start_time = time::SteadyTime::now();

    println!("Started rendering at: {}", start_time.rfc822());

    let context = RenderContext {
        options,
        scene,
        photon_map,
    };

    let width = options.width;
    let height = options.height;

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
        for _ in 0..height {
            let _ = rx.recv();
            pb.inc();
        }
    } else {
        for y in 0..height {
            let row = (0..width).map(|x| color_at_pixel(&context, x, y)).collect();
            let mut results = results.lock().unwrap();
            results[y as usize] = row;
            pb.inc();
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
