use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use wavefront_obj;

use color::Color;
use direction::Direction;
use image;
use lights::Light;
use matrix::Matrix44f;
use object::{DEFAULT_ALBEDO, Object};
use point::Point;
use shader::{IOR_GLASS, Shader};
use shapes::{Composite, Mesh, MeshTriangle, Plane, Shape, Sphere};
use system::{Camera, Transformable};
use texture::{Pattern,Texture};

#[derive(Debug, Clone)]
pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

pub fn setup_scene(w: u32, h: u32) -> Scene {
    let mut camera = Camera::new(Point::new(2.0, 5.0, 9.0), 60.0);
    camera.look_at(Point::new(-1.0, 2.0, 0.0));

    let lights: Vec<Light> = vec![
        Light::Point { color: Color::white(), intensity: 3000.0, origin: Point::new(-3.0, 8.0, 9.0) },
        Light::Point { color: Color::white(), intensity: 3000.0, origin: Point::new(4.0, 8.0, 9.0) },
//        Light::Distant { color: Color::white(), intensity: 1.0, direction: Direction::new(0.0, 0.0, -1.0) },
//        Light::Point { color: Color::white(), intensity: 10000.0, origin: Point::new(10.0, 10.0, -10.0) },
//        Light::Point { color: Color::white(), intensity: 8000.0, origin: Point::new(-10.0, 0.0, -5.0) },
//        Light::Point { color: Color::white(), intensity: 5000.0, origin: Point::new(10.0, 10.0, -15.0) },
//        Light::Point { color: Color::white(), intensity: 5000.0, origin: Point::new(-10.0, 0.0, -30.0) },
    ];

    let obj = {
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

    let matte_white = vec![
        (1.0, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::white()),
            roughness: 0.0,
            highlight: 0.0,
        })];

    let matte_blue = vec![
        (1.0, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::blue()),
            roughness: 0.0,
            highlight: 0.0,
        })];

    let matte_red = vec![
        (1.0, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::red()),
            roughness: 0.0,
            highlight: 0.0,
        })];

    let shiny_white = vec![
        (0.8, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::white()),
            roughness: 0.2,
            highlight: 50.0,
        }),
        (0.2, Shader::Reflection)
    ];

    let shiny_red = vec![
        (0.8, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::red()),
            roughness: 0.2,
            highlight: 50.0,
        }),
        (0.2, Shader::Reflection)
    ];

    let shiny_green = vec![
        (0.8, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::green()),
            roughness: 0.2,
            highlight: 50.0,
        }),
        (0.2, Shader::Reflection)
    ];

    let shiny_blue = vec![
        (0.8, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Solid(Color::blue()),
            roughness: 0.2,
            highlight: 50.0,
        }),
        (0.2, Shader::Reflection)
    ];

    let transparent = vec![
        (1.0, Shader::Transparency { ior: IOR_GLASS }),
    ];

    let matte_black_white_checkboard = vec![
        (1.0, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Pattern(Pattern::Checkerboard(Color::black(), Color::white(), 3.0)),
            roughness: 0.0,
            highlight: 0.0,
        })
    ];

    let shiny_black_white_checkboard = vec![
        (0.8, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Pattern(Pattern::Checkerboard(Color::black(), Color::white(), 0.5)),
            roughness: 0.2,
            highlight: 50.0,
        }),
        (0.2, Shader::Reflection)
    ];

    let earth_image = {
        let f = File::open("earth.jpg").expect("could not open earth.jpg");
        let r = BufReader::new(f);
        image::load(r, image::JPEG).expect("could not decode earth.jpg")
    };

    let earth = vec![
        (1.0, Shader::DiffuseSpecular {
            albedo: DEFAULT_ALBEDO,
            texture: Texture::Image(earth_image, 1.0),
            roughness: 0.0,
            highlight: 0.0,
        })
    ];

    let objects: Vec<Object> = vec![
        Object::new(
            "bottom plane",
            Shape::Plane(Plane::new(Direction::new(0.0, 1.0, 0.0))),
            shiny_black_white_checkboard.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, 0.0))),

        Object::new(
            "back plane",
            Shape::Plane(Plane::new(Direction::new(0.0, 0.0, 1.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, -5.0))),

        Object::new(
            "left plane",
            Shape::Plane(Plane::new(Direction::new(1.0, 0.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(-5.0, 0.0, 0.0))),

        Object::new(
            "right plane",
            Shape::Plane(Plane::new(Direction::new(-1.0, 0.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(5.0, 0.0, 0.0))),

        Object::new(
            "top plane",
            Shape::Plane(Plane::new(Direction::new(0.0, -1.0, 0.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 10.0, 0.0))),

        Object::new(
            "front plane",
            Shape::Plane(Plane::new(Direction::new(0.0, 0.0, -1.0))),
            matte_white.clone()
        ).transform(Matrix44f::translation(Direction::new(0.0, 0.0, 10.0))),

        Object::new(
            "linked torus",
            Shape::Composite(obj),
            matte_white.clone()
        ).transform(
            Matrix44f::rotation_y(-30.0) *
//            Matrix44f::scaling(Direction::new(1.5, 1.5, 1.5)) *
            Matrix44f::translation(Direction::new(-3.0, 1.5, 4.0))
        ),

        Object::new(
            "earth",
            Shape::Sphere(Sphere::new(2.0)),
            earth.clone()
        ).transform(
            Matrix44f::rotation_y(-90.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

        Object::new(
            "sphere1",
            Shape::Sphere(Sphere::new(1.0)),
            shiny_red.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)) *
            Matrix44f::rotation_z(45.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

        Object::new(
            "sphere2",
            Shape::Sphere(Sphere::new(1.0)),
            shiny_green.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 5.0, 0.0))
        ),

        Object::new(
            "sphere3",
            Shape::Sphere(Sphere::new(1.0)),
            shiny_blue.clone()
        ).transform(
            Matrix44f::translation(Direction::new(0.0, 3.0, 0.0)) *
            Matrix44f::rotation_z(-45.0) *
            Matrix44f::translation(Direction::new(0.0, 2.0, 0.0))
        ),

//        Object::new(
//            "sphere5",
//            Shape::Sphere(Sphere::new(2.0)),
//            transparent.clone()
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
