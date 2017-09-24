use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use wavefront_obj;

use color::Color;
use direction::Direction;
use image;
use lights::Light;
use lights::omni::Omni;
use materials::glass::Glass;
use materials::matte::Matte;
use materials::plastic::Plastic;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use shapes::Shape;
use shapes::sphere::Sphere;
use shapes::plane::Plane;
use shapes::composite::Composite;
use shapes::mesh::{Mesh,MeshTriangle};
use system::{Camera, Transformable};
use texture::{Pattern,Texture};

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Box<Light>>,
    pub objects: Vec<Object>,
}

pub fn setup_scene<'a>(w: u32, h: u32) -> Scene {
    let camera = Camera::new(Point::new(2.0, 5.0, 9.0), 60.0).look_at(Point::new(-1.0, 2.0, 0.0));

    let lights: Vec<Box<Light>> = vec![
        Box::new(Omni { color: Color::white(), intensity: 3000.0, origin: Point::new(-3.0, 8.0, 9.0) }),
        Box::new(Omni { color: Color::white(), intensity: 3000.0, origin: Point::new(4.0, 8.0, 9.0) }),
//        Light::Distant { color: Color::white(), intensity: 1.0, direction: Direction::new(0.0, 0.0, -1.0) },
//        Light::Point { color: Color::white(), intensity: 10000.0, origin: Point::new(10.0, 10.0, -10.0) },
//        Light::Point { color: Color::white(), intensity: 8000.0, origin: Point::new(-10.0, 0.0, -5.0) },
//        Light::Point { color: Color::white(), intensity: 5000.0, origin: Point::new(10.0, 10.0, -15.0) },
//        Light::Point { color: Color::white(), intensity: 5000.0, origin: Point::new(-10.0, 0.0, -30.0) },
    ];

    let obj: Box<Shape> = {
        print!("Loading object file...");
        let mut obj_file_contents = String::new();
        let mut obj_file = File::open("LinkedTorus.obj").expect("could not open object file");
        obj_file.read_to_string(&mut obj_file_contents).expect("could not read object file");
        let obj_set = wavefront_obj::obj::parse(obj_file_contents).expect("Could not parse object file!");
        println!(" done.");
        println!("# objects: {}", obj_set.objects.len());
        print!("Converting objects...");
        let obj = convert_objs(&obj_set);
        println!(" done.");
        //        println!("{:?}", obj);
        obj
    };

    let matte_white = Box::new(Matte::new(Texture::Solid(Color::white())));
//    let matte_blue = Box::new(Matte::new(Texture::Solid(Color::blue())));
//    let matte_red = Box::new(Matte::new(Texture::Solid(Color::red())));

//    let plastic_white = Box::new(Plastic::new(Texture::Solid(Color::white())));
    let plastic_red = Box::new(Plastic::new(Texture::Solid(Color::red())));
    let plastic_green = Box::new(Plastic::new(Texture::Solid(Color::green())));
    let plastic_blue = Box::new(Plastic::new(Texture::Solid(Color::blue())));

    let glass = Box::new(Glass::new());

    let plastic_checkerboard = Box::new(Plastic::new(Texture::Pattern(Pattern::Checkerboard(Color::black(), Color::white(), 0.5))));

    let earth_image = {
        let f = File::open("earth.jpg").expect("could not open earth.jpg");
        let r = BufReader::new(f);
        image::load(r, image::JPEG).expect("could not decode earth.jpg")
    };

    let earth = Box::new(Matte::new(Texture::Image(earth_image, 1.0)));

    let objects: Vec<Object> = vec![
        Object::new(
            "bottom plane",
            Box::new(Plane::new(Direction::new(0.0, 1.0, 0.0))),
            plastic_checkerboard
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, 0.0))),

        Object::new(
            "back plane",
            Box::new(Plane::new(Direction::new(0.0, 0.0, 1.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, -5.0))),

        Object::new(
            "left plane",
            Box::new(Plane::new(Direction::new(1.0, 0.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(-5.0, 0.0, 0.0))),

        Object::new(
            "right plane",
            Box::new(Plane::new(Direction::new(-1.0, 0.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(5.0, 0.0, 0.0))),

        Object::new(
            "top plane",
            Box::new(Plane::new(Direction::new(0.0, -1.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 10.0, 0.0))),

        Object::new(
            "front plane",
            Box::new(Plane::new(Direction::new(0.0, 0.0, -1.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, 10.0))),

        Object::new(
            "linked torus",
            obj,
            matte_white.clone()
        ).transform(
            Matrix44f::rotation_y(-30.0) *
//            Matrix44f::scaling(Direction::new(1.5, 1.5, 1.5)) *
            Matrix44f::translation(Direction::new(-3.0, 1.5, 4.0))
        ),

        Object::new(
            "earth",
            Box::new(Sphere::new(2.0)),
            earth.clone()
        ).transform(
            Matrix44f::rotation_y(-90.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

        Object::new(
            "sphere1",
            Box::new(Sphere::new(1.0)),
            plastic_red.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)) *
            Matrix44f::rotation_z(45.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

        Object::new(
            "sphere2",
            Box::new(Sphere::new(1.0)),
            plastic_green.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 5.0, 0.0))
        ),

        Object::new(
            "sphere3",
            Box::new(Sphere::new(1.0)),
            plastic_blue.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)) *
            Matrix44f::rotation_z(-45.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

        Object::new(
            "sphere5",
            Box::new(Sphere::new(1.0)),
            glass.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 1.0, 3.0))),
    ];

    Scene {
        camera,
        lights,
        objects,
    }
}

fn convert_objs(objs: &wavefront_obj::obj::ObjSet) -> Box<Shape> {
    let shapes: Vec<Mesh> = objs.objects.iter().map(|o| {
        let vertices = o.vertices.iter().map(|v| Point::new(v.x, v.y, v.z)).collect();
        let normals = o.normals.iter().map(|n| Direction::new(n.x, n.y, n.z)).collect();
        let triangles = o.geometry
            .iter()
            .flat_map(|g| &g.shapes)
            .flat_map(|s| match s.primitive {
                wavefront_obj::obj::Primitive::Triangle(v0, v1, v2) => Some(MeshTriangle {
                    vertex_indices: [v0.0, v1.0, v2.0],
                    normal_indices: [v0.2.unwrap(), v1.2.unwrap(), v2.2.unwrap()],
                }),
                _ => None,
            })
            .collect();

        Mesh::new(vertices, normals, triangles, true)
    }).collect();

    let shapes: Vec<Box<Shape>> = shapes.into_iter().map(|m| Box::new(m) as Box<Shape>).collect();

    Box::new(Composite::new(shapes))
}
