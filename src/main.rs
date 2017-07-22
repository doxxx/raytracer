extern crate image;

mod vector;
mod shapes;
mod system;
mod material;

use std::fs::File;
use std::path::Path;
use std::cmp::Ordering;
use std::f64;

use vector::Vector3f;
use shapes::{Shape, Sphere};
use system::{Camera, Ray};
use material::{Color,Checkerboard};


// struct Ray {

// }

// impl Ray {
//     fn new(sx: u32, sy: u32) -> Ray {
//         Ray{}
//     }
// }

fn color_to_pixel(v: Color) -> image::Rgb<u8> {
    image::Rgb([(v.0 * 255.0) as u8, (v.1 * 255.0) as u8, (v.2 * 255.0) as u8])
}

fn cast_ray(ray: Ray, objects: &Vec<Box<Shape>>) -> Option<(&Box<Shape>, f64)> {
    objects.iter()
            .flat_map(|o| o.intersect(ray.origin, ray.direction).map(|i| (o, i)))
            .min_by(|&(_, i1), &(_, i2)| i1.partial_cmp(&i2).unwrap())

    // let mut nearest = f64::MAX;
    // let mut nearest_object = None;

    // for object in &objects {
    //     if let Some(i) = object.intersect(ray.origin, ray.direction) {
    //         if i < nearest {
    //             nearest = i;
    //             nearest_object = Some(object);
    //         }
    //     }
    // }
}

fn main() {
    let w = 640;
    let h = 480;
    let mut imgbuf = image::RgbImage::new(w, h);
    let camera = Camera::new(w, h, 60.0);

    let white = Vector3f(1.0, 1.0, 1.0);
    let blue = Vector3f(0.5, 0.5, 1.0);

    let white_checkboard = Checkerboard::new(white, white * 0.8, 4.0);
    let blue_checkboard = Checkerboard::new(blue, blue * 0.8, 4.0);

    let objects: Vec<Box<Shape>> = vec![
        Box::new(Sphere::new(Vector3f(0.0, 0.0, -20.0), 1.0, Box::new(blue_checkboard))),
        Box::new(Sphere::new(Vector3f(0.0, 6.0, -20.0), 2.0, Box::new(white_checkboard))),
        Box::new(Sphere::new(Vector3f(-4.0, 4.0, -25.0), 4.0, Box::new(blue_checkboard))),
        Box::new(Sphere::new(Vector3f(4.0, -4.0, -25.0), 6.0, Box::new(white_checkboard))),
        Box::new(Sphere::new(Vector3f(-6.0, -4.0, -20.0), 2.0, Box::new(blue_checkboard))),
    ];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = camera.pixel_ray(x, y);
        let intersection = cast_ray(ray, &objects);

        if let Some((object, t)) = intersection {
            let (normal, texture_coords) = object.surface_data(ray.project(t));
            let color = object.material().color(ray.direction, normal, texture_coords);
            *pixel = color_to_pixel(color);
        }

    }

    let ref mut fout = File::create(&Path::new("out.png")).unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
