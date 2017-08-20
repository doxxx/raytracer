use std::fs::File;
use std::io::Read;

use wavefront_obj;

use color::Color;
use direction::Direction;
use lights::{DistantLight, Light, PointLight};
use material::{IOR_GLASS, Material};
use matrix::Matrix44f;
use object::{DEFAULT_ALBEDO, Object};
use point::Point;
use shapes::{Composite, Mesh, MeshTriangle, Plane, Shape, Sphere};
use system::{Camera, Options, render};

#[derive(Debug, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

pub fn setup_scene(w: u32, h: u32) -> Scene {
    let white = Color::new(1.0, 1.0, 1.0);
    let red = Color::new(1.0, 0.0, 0.0);
    let green = Color::new(0.0, 1.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0);

    let mut camera = Camera::new(w, h, 60.0);
//    camera.transform(Matrix44f::rotation_x(-10.0));
//    camera.transform(Matrix44f::rotation_y(10.0));
//    camera.transform(Matrix44f::translation(Direction::new(3.0, 3.0, 0.0)));

    let lights: Vec<Light> = vec![
        Light::Distant(DistantLight::new(
            white,
            2.0,
            Direction::new(-1.0, -1.0, -1.0).normalize(),
        )),
        Light::Point(PointLight::new(blue, 5000.0, Point::new(-10.0, 10.0, -15.0))),
        Light::Point(PointLight::new(red, 5000.0, Point::new(10.0, 10.0, -15.0))),
    ];

    let obj = {
        print!("Loading object file...");
        let mut obj_file_contents = String::new();
        let mut obj_file = File::open("Monkey.obj").expect("could not open object file");
        obj_file.read_to_string(&mut obj_file_contents).expect("could not read object file");
        let obj_set = wavefront_obj::obj::parse(obj_file_contents).expect("Could not parse object file!");
        println!(" done.");
//        println!("{:?}", obj_set.objects[0]);
        print!("Converting objects...");
        let obj = convert_objs(&obj_set);
        println!(" done.");
//        println!("{:?}", obj);
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
            Shape::Composite(obj),
            DEFAULT_ALBEDO,
            Material::Diffuse(white),
        )/*.transform(Matrix44f::rotation_y(20.0))*/.transform(Matrix44f::scaling(Direction::new(1.5, 1.5, 1.5))).transform(Matrix44f::translation(Direction::new(6.0, -2.0, -15.0))),
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
//        Object::new(
//            "sphere6",
//            Shape::Sphere(Sphere::new(2.0)),
//            DEFAULT_ALBEDO,
//            Material::ReflectiveAndRefractive(IOR_GLASS)
//        ).transform(Matrix44f::translation(Direction::new(-1.0, -1.0, -10.0))),
    ];

    Scene {
        camera: camera,
        lights: lights,
        objects: objects,
    }
}

fn convert_objs(objs: &wavefront_obj::obj::ObjSet) -> Composite {
    Composite::new(objs.objects.iter().map(|o| {
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

        Shape::Mesh(Mesh::new(vertices, normals, triangles, true))
    }).collect())
}
