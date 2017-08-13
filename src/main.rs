extern crate image;
extern crate clap;
extern crate wavefront_obj;
extern crate pbr;
extern crate num_cpus;

mod color;
mod direction;
mod lights;
mod material;
mod matrix;
mod object;
mod point;
mod shapes;
mod system;
mod texture;
mod vector;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;
use std::io::Stdout;

use clap::{App, Arg};
use pbr::ProgressBar;

use color::Color;
use direction::Direction;
use lights::{DistantLight, Light, PointLight};
use material::{IOR_GLASS, Material};
use matrix::Matrix44f;
use object::{DEFAULT_ALBEDO, Object};
use point::Point;
use shapes::{Mesh, MeshTriangle, Plane, Shape, Sphere};
use system::{Camera, Options, calculate_pixel_color};

fn color_to_pixel(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

 fn convert_obj(o: &wavefront_obj::obj::Object) -> Mesh {
     let vertices = o.vertices.iter().map(|v| Point::new(v.x, v.y, v.z)).collect();
     let triangles = o.geometry
         .iter()
         .flat_map(|g| &g.shapes)
         .flat_map(|s| match s.primitive {
             wavefront_obj::obj::Primitive::Triangle(v0, v1, v2) => Some(MeshTriangle {
                 indices: [v0.0, v1.0, v2.0],
             }),
             _ => None,
         })
         .collect();

     Mesh {
         vertices: vertices,
         triangles: triangles,
     }
 }

fn main() {
    let default_cpus = format!("{}", num_cpus::get() - 1);
    let app = App::new("raytracer")
        .version("0.1.0")
        .author("Gordon Tyler <gordon@doxxx.net>")
        .about("Simple ray tracer")
        .arg(
            Arg::with_name("width")
                .short("w")
                .value_name("WIDTH")
                .help("Image width")
                .takes_value(true)
                .default_value("1024"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .value_name("HEIGHT")
                .help("Image height")
                .takes_value(true)
                .default_value("768"),
        )
        .arg(
            Arg::with_name("num_threads")
                .short("t")
                .value_name("THREADS")
                .help("Number of render threads")
                .takes_value(true)
                .validator(|s| {
                    if s.parse::<usize>().is_ok() { return Ok(()); }
                    Err(String::from("The value must be a number."))
                })
                .default_value(&default_cpus)
        );
    let options = app.get_matches();

    let w: u32 = match options.value_of("width").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad width!");
            return;
        }
    };
    let h: u32 = match options.value_of("height").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad height!");
            return;
        }
    };

    let options = Options {
        num_threads: options.value_of("num_threads").unwrap().parse().unwrap(),
        width: w,
        height: h,
        background_color: Color::new(0.1, 0.1, 0.5),
        bias: 1e-4,
        max_depth: 5,
    };

    let mut camera = Camera::new(w, h, 60.0);
    camera.transform(Matrix44f::rotation_x(-15.0));
    camera.transform(Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)));

    let white = Color::new(1.0, 1.0, 1.0);
    let red = Color::new(1.0, 0.0, 0.0);
    let green = Color::new(0.0, 1.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0);

    let obj = {
        print!("Loading object file...");
        let mut obj_file_contents = String::new();
        let mut obj_file = std::fs::File::open("LinkedTorus.obj").expect("could not open object file");
        obj_file.read_to_string(&mut obj_file_contents).expect("could not read object file");
        let obj_set = wavefront_obj::obj::parse(obj_file_contents).expect("Could not parse object file!");
        println!(" done.");
        print!("Converting object...");
        let obj = convert_obj(&obj_set.objects[0]);
        println!(" done.");
        obj
    };

    let objects: Vec<Object> = vec![
        Object::new(
            "plane",
            Shape::Plane(Plane::new(Direction::new(0.0, 1.0, 0.0))),
            DEFAULT_ALBEDO,
            Material::Diffuse(white)
        ).transform(Matrix44f::translation(Direction::new(0.0, -5.0, 0.0))),
        Object::new(
            "object",
            Shape::Mesh(obj),
            DEFAULT_ALBEDO,
            Material::Diffuse(white),
        ).transform(Matrix44f::translation(Direction::new(0.0, 2.0, -20.0))),
//        Object::new(
//            "sphere2",
//            Shape::Sphere(Sphere::new(2.0)),
//            DEFAULT_ALBEDO,
//            Material::Diffuse(white)
//        ).transform(Matrix44f::translation(Direction::new(0.0, 6.0, -24.0))),
//        Object::new(
//            "sphere3",
//            Shape::Sphere(Sphere::new(4.0)),
//            DEFAULT_ALBEDO,
//            Material::Diffuse(white)
//        ).transform(Matrix44f::translation(Direction::new(-4.0, 4.0, -25.0))),
//        Object::new(
//            "sphere4",
//            Shape::Sphere(Sphere::new(6.0)),
//            DEFAULT_ALBEDO,
//            Material::Reflective
//        ).transform(Matrix44f::translation(Direction::new(4.0, -4.0, -25.0))),
//        Object::new(
//            "sphere5",
//            Shape::Sphere(Sphere::new(2.0)),
//            DEFAULT_ALBEDO,
//            Material::Diffuse(white)
//        ).transform(Matrix44f::translation(Direction::new(-6.0, -3.0, -20.0))),
//        Object::new(
//            "sphere6",
//            Shape::Sphere(Sphere::new(2.0)),
//            DEFAULT_ALBEDO,
//            Material::ReflectiveAndRefractive(IOR_GLASS)
//        ).transform(Matrix44f::translation(Direction::new(-1.0, -1.0, -10.0))),
    ];

    let lights: Vec<Light> = vec![
        Light::Distant(DistantLight::new(
            white,
            1.0,
            Direction::new(0.0, -1.0, 0.0).normalize(),
        )),
        Light::Point(PointLight::new(blue, 5000.0, Point::new(-10.0, 10.0, -15.0))),
        Light::Point(PointLight::new(red, 5000.0, Point::new(10.0, 10.0, -15.0))),
    ];

    let imgbuf = render(options, camera, objects, lights);

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

fn render(
    options: Options,
    camera: Camera,
    objects: Vec<Object>,
    lights: Vec<Light>,
) -> image::RgbImage {
    let mut imgbuf = image::RgbImage::new(options.width, options.height);
    let width = options.width;
    let height = options.height;
    let rows: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new((0..height).collect()));
    let mut results: Vec<Vec<Color>> = Vec::with_capacity(height as usize);
    results.resize(height as usize, Vec::new());
    let results: Arc<Mutex<Vec<Vec<Color>>>> = Arc::new(Mutex::new(results));

    // start progress bar update task
    let mut pb: ProgressBar<Stdout> = ProgressBar::new(height as u64);
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    pb.message(&format!("Rendering (x{}): ", options.num_threads));

    // spawn threads for rendering rows
    // each thread sends the row index when it finishes rendering
    for _ in 0..options.num_threads {
        let options = options.clone();
        let camera = camera.clone();
        let objects = objects.to_vec();
        let lights = lights.to_vec();
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
                                calculate_pixel_color(&options, &camera, &objects, &lights, x, y)
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

    pb.finish();

    let results = results.lock().unwrap();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color_to_pixel(results[y as usize][x as usize]);
    }

    imgbuf
}
