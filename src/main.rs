extern crate image;
extern crate clap;

mod lights;
mod object;
mod shapes;
mod system;
mod texture;
mod vector;

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

use lights::{DistantLight, Light, PointLight};
use object::{DEFAULT_ALBEDO, IOR_GLASS, Material, Object};
use shapes::{Plane, Sphere};
use system::{Camera, Color, Options, calculate_pixel_color};
// use texture::{Checkerboard, Flat};
use vector::Vector3f;

fn color_to_pixel(v: Color) -> image::Rgb<u8> {
    let r = (v.0 * 255.0).min(255.0) as u8;
    let g = (v.1 * 255.0).min(255.0) as u8;
    let b = (v.2 * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn main() {
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
        );
    let options = app.get_matches();

    let w = match options.value_of("width").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad width!");
            return;
        }
    };
    let h = match options.value_of("height").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad height!");
            return;
        }
    };

    let mut imgbuf = image::RgbImage::new(w, h);
    let camera = Camera::new(w, h, 60.0);

    let white = Vector3f(1.0, 1.0, 1.0);
    let red = Vector3f(1.0, 0.0, 0.0);
    let green = Vector3f(0.0, 1.0, 1.0);
    let blue = Vector3f(0.0, 0.0, 1.0);

    // let white_flat = Flat::new(white);
    // let red_flat = Flat::new(red);
    // let blue_flat = Flat::new(blue);
    // let white_checkboard = Checkerboard::new(white, white * 0.8, 4.0);
    // let white_checkboard_large = Checkerboard::new(white, white * 0.8, 0.5);
    // let blue_checkboard = Checkerboard::new(blue, blue * 0.8, 4.0);
    // let red_checkboard = Checkerboard::new(red, red * 0.8, 0.1);

    let objects: Vec<Object> = vec![
        Object::new(
            "plane",
            Box::new(Plane::new(
                Vector3f(0.0, -5.0, 0.0),
                Vector3f(0.0, 1.0, 0.0),
            )),
            DEFAULT_ALBEDO,
            Material::Diffuse(green),
        ),
        Object::new(
            "sphere2",
            Box::new(Sphere::new(Vector3f(0.0, 6.0, -24.0), 2.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white),
        ),
        Object::new(
            "sphere3",
            Box::new(Sphere::new(Vector3f(-4.0, 4.0, -25.0), 4.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white),
        ),
        Object::new(
            "sphere4",
            Box::new(Sphere::new(Vector3f(4.0, -4.0, -25.0), 6.0)),
            DEFAULT_ALBEDO,
            Material::Reflective,
        ),
        Object::new(
            "sphere5",
            Box::new(Sphere::new(Vector3f(-6.0, -3.0, -20.0), 2.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white),
        ),
        Object::new(
            "sphere6",
            Box::new(Sphere::new(Vector3f(-1.0, -1.0, -10.0), 2.0)),
            DEFAULT_ALBEDO,
            Material::ReflectiveAndRefractive(IOR_GLASS),
        )
    ];

    let lights: Vec<Box<Light>> = vec![
        Box::new(DistantLight::new(
            white,
            1.0,
            Vector3f(0.0, -1.0, 0.0).normalize(),
        )),
        Box::new(PointLight::new(blue, 5000.0, Vector3f(-10.0, 10.0, -15.0))),
        Box::new(PointLight::new(red, 5000.0, Vector3f(10.0, 10.0, -15.0))),
    ];

    let options = Options {
        background_color: Vector3f(0.1, 0.1, 0.5),
        bias: 1e-4,
        max_depth: 5,
    };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color_to_pixel(calculate_pixel_color(
            &options,
            &camera,
            &objects,
            &lights,
            x,
            y,
        ));
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
