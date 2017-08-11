extern crate image;
extern crate clap;
extern crate rayon;

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
use std::path::Path;

use clap::{App, Arg};
use rayon::prelude::*;

use color::Color;
use direction::Direction;
use lights::{DistantLight, Light, PointLight};
use material::{IOR_GLASS, Material};
use matrix::Matrix44f;
use object::{DEFAULT_ALBEDO, Object};
use point::Point;
use shapes::{Plane, Shape, Sphere};
use system::{Camera, Options, calculate_pixel_color};

fn color_to_pixel(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn main() {
    let app = App::new("raytracer")
        .version("0.1.0")
        .author("Gordon Tyler <gordon@doxxx.net>")
        .about("Simple ray tracer")
        .arg(Arg::with_name("parallel").short("p").help(
            "Use parallel rendering",
        ))
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
    let parallel: bool = options.is_present("parallel");

    let mut imgbuf = image::RgbImage::new(w, h);

    let mut camera = Camera::new(w, h, 60.0);
    camera.transform(Matrix44f::rotation_x(-15.0));
    camera.transform(Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)));

    let white = Color::new(1.0, 1.0, 1.0);
    let red = Color::new(1.0, 0.0, 0.0);
    let green = Color::new(0.0, 1.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0);

    let objects: Vec<Object> = vec![
        Object::new(
            "plane",
            Shape::Plane(Plane::new(Direction::new(0.0, 1.0, 0.0))),
            DEFAULT_ALBEDO,
            Material::Diffuse(white)
        ).transform(Matrix44f::translation(Direction::new(0.0, -5.0, 0.0))),
        Object::new(
            "sphere2",
            Shape::Sphere(Sphere::new(2.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white)
        ).transform(Matrix44f::translation(Direction::new(0.0, 6.0, -24.0))),
        Object::new(
            "sphere3",
            Shape::Sphere(Sphere::new(4.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white)
        ).transform(Matrix44f::translation(Direction::new(-4.0, 4.0, -25.0))),
        Object::new(
            "sphere4",
            Shape::Sphere(Sphere::new(6.0)),
            DEFAULT_ALBEDO,
            Material::Reflective
        ).transform(Matrix44f::translation(Direction::new(4.0, -4.0, -25.0))),
        Object::new(
            "sphere5",
            Shape::Sphere(Sphere::new(2.0)),
            DEFAULT_ALBEDO,
            Material::Diffuse(white)
        ).transform(Matrix44f::translation(Direction::new(-6.0, -3.0, -20.0))),
        Object::new(
            "sphere6",
            Shape::Sphere(Sphere::new(2.0)),
            DEFAULT_ALBEDO,
            Material::ReflectiveAndRefractive(IOR_GLASS)
        ).transform(Matrix44f::translation(Direction::new(-1.0, -1.0, -10.0))),
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

    let options = Options {
        background_color: Color::new(0.1, 0.1, 0.5),
        bias: 1e-4,
        max_depth: 5,
    };

    if parallel {
        render_parallel(&mut imgbuf, &options, &camera, &objects, &lights);
    } else {
        render_serial(&mut imgbuf, &options, &camera, &objects, &lights);
    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}

fn render_parallel(
    imgbuf: &mut image::RgbImage,
    options: &Options,
    camera: &Camera,
    objects: &[Object],
    lights: &[Light],
) {
    let width = imgbuf.width();
    let height = imgbuf.height();
    let rows: Vec<u32> = (0..height).collect();
    let colors: Vec<Color> = rows.par_iter()
        .flat_map(|y| -> Vec<Color> {
            (0..width)
                .map(|x| {
                    calculate_pixel_color(&options, &camera, &objects, &lights, x, *y)
                })
                .collect()
        })
        .collect();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let offset = (y * width + x) as usize;
        *pixel = color_to_pixel(colors[offset]);
    }
}

fn render_serial(
    imgbuf: &mut image::RgbImage,
    options: &Options,
    camera: &Camera,
    objects: &[Object],
    lights: &[Light],
) {
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
}
