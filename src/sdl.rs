use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use image;
use wavefront_obj;

use direction::Direction;
use lights::Light;
use materials::Material;
use matrix::Matrix44f;
use object::Object;
use point::Point;
use sdl_grammar;
use shapes::Shape;
use shapes::bounding_box::BoundingBox;
use shapes::composite::Composite;
use shapes::mesh::{Mesh, MeshTriangle};
use system::{Camera, Transformable};

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Box<Light>>,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn bounding_box(&self) -> BoundingBox {
        self.objects.iter().fold(BoundingBox::zero(), |acc, ref obj| acc.extend(&obj.bounding_box()))
    }
}

pub fn parse(s: &str) -> sdl_grammar::ParseResult<Scene> {
    sdl_grammar::scene(s)
}

pub fn new_object(loc: Point, shape: Box<Shape>, material: Box<Material>, transform: Option<Matrix44f>) -> Object {
    Object::new("object", shape, material)
        .transform(transform.unwrap_or(Matrix44f::identity()))
        .transform(Matrix44f::translation(Direction::new(loc.x, loc.y, loc.z)))
}

pub fn load_image(path: &str) -> image::DynamicImage {
    let f = File::open(path).expect("could not open image file");
    let r = BufReader::new(f);
    image::load(r, image::JPEG).expect("could not decode image file")
}

pub fn load_mesh_file(path: &str) -> Box<Shape> {
    let mut obj_file = File::open(path).expect("could not open object file");
    let mut obj_file_contents = String::new();
    obj_file.read_to_string(&mut obj_file_contents).expect("could not read object file");
    let obj_set = wavefront_obj::obj::parse(obj_file_contents).expect("Could not parse object file!");
    convert_objs(&obj_set)
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

pub fn combine_transforms(transforms: Vec<Matrix44f>) -> Matrix44f {
    transforms.iter().fold(Matrix44f::identity(), |acc, &m| acc * m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_bounding_box() {
        let s = Scene {
            camera: Camera::new(Point::zero(), 60.0),
            lights: Vec::new(),
            objects: vec![
                Object::new(
                    "sphere",
                    Box::new(::shapes::sphere::Sphere::new(1.0)),
                    Box::new(::materials::matte::Matte::new(::texture::Texture::Solid(::color::Color::white())))
                ).transform(Matrix44f::translation(Direction::new(1.5, 0.0, 0.0))),
                Object::new(
                    "sphere",
                    Box::new(::shapes::sphere::Sphere::new(1.0)),
                    Box::new(::materials::matte::Matte::new(::texture::Texture::Solid(::color::Color::white())))
                ).transform(Matrix44f::translation(Direction::new(-1.5, 0.0, 0.0))),
            ],
        };

        let bb = s.bounding_box();
        assert_eq!(bb, BoundingBox::new(Point::new(-2.5, -1.0, -1.0), Point::new(2.5, 1.0, 1.0)));
    }
}
