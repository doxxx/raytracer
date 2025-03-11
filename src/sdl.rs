use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use image;
use wavefront_obj;

use crate::color::Color;
use crate::direction::Direction;
use crate::materials::Material;
use crate::matrix::Matrix44f;
use crate::object::Object;
use crate::point::Point;
use crate::sdl_grammar;
use crate::shapes::{Composite, Mesh, MeshTriangle, Shape};
use crate::system::{Camera, Options};

pub struct Scene {
    pub options: SceneOptions,
    pub camera: Camera,
    pub objects: Vec<Object>,
}

pub struct SceneOptions {
    pub background_color: Color,
}

impl SceneOptions {
    pub fn default() -> SceneOptions {
        SceneOptions {
            background_color: Color::black(),
        }
    }
}

pub fn parse(options: &Options, s: &str) -> Result<Scene, String> {
    sdl_grammar::sdl_grammar::scene(&s, &options).map_err(|err| err.to_string())
}

pub fn new_object(name: Option<String>, shape: Box<dyn Shape>, material: Box<dyn Material>) -> Object {
    Object::new(&name.unwrap_or(String::from("object")), shape, material)
}

pub fn transform_shape(mut shape: Box<dyn Shape>, transform: Option<Matrix44f>) -> Box<dyn Shape> {
    shape.transform(transform.unwrap_or(Matrix44f::identity()));
    shape
}

pub fn load_image(path: &str) -> image::DynamicImage {
    let f = File::open(path).expect("could not open image file");
    let r = BufReader::new(f);
    image::load(r, image::JPEG).expect("could not decode image file")
}

pub fn load_mesh_file(path: &str) -> Box<dyn Shape> {
    let mut obj_file = File::open(path).expect("could not open object file");
    let mut obj_file_contents = String::new();
    obj_file
        .read_to_string(&mut obj_file_contents)
        .expect("could not read object file");
    let obj_set = wavefront_obj::obj::parse(obj_file_contents).expect("Could not parse object file!");
    convert_objs(&obj_set)
}

fn convert_objs(objs: &wavefront_obj::obj::ObjSet) -> Box<dyn Shape> {
    let shapes: Vec<Mesh> = objs
        .objects
        .iter()
        .map(|o| {
            let vertices = o.vertices.iter().map(|v| Point::new(v.x, v.y, v.z)).collect();
            let normals = o.normals.iter().map(|n| Direction::new(n.x, n.y, n.z)).collect();
            let triangles = o
                .geometry
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
        })
        .collect();

    let shapes: Vec<Box<dyn Shape>> = shapes.into_iter().map(|m| Box::new(m) as Box<dyn Shape>).collect();

    Box::new(Composite::new(shapes))
}

pub fn combine_transforms(transforms: Vec<Matrix44f>) -> Matrix44f {
    transforms.iter().fold(Matrix44f::identity(), |acc, &m| acc * m)
}
