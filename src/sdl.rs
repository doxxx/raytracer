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
use shapes::composite::Composite;
use shapes::mesh::{Mesh,MeshTriangle};
use system::{Camera,Transformable};

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Box<Light>>,
    pub objects: Vec<Object>,
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
    use sdl_grammar::*;

    use color::Color;
    use direction::Direction;
    use lights::Light;
    use matrix::Matrix44f;
    use object::Object;
    use point::Point;
    use shader::{DEFAULT_ALBEDO,Shader};
    use shapes::{Shape,Sphere};
    use system::{Camera,Transformable};
    use texture::Texture;

    #[test]
    pub fn parse_float() {
        assert_eq!(Ok(123.456), float("123.456"));
        assert_eq!(Ok(-123.456), float("-123.456"));
    }

    #[test]
    pub fn parse_uint() {
        assert_eq!(Ok(123), uint("123"));
    }

    #[test]
    pub fn parse_point() {
        assert_eq!(Ok(Point::new(1.0, 2.0, 3.0)), point("<1.0,2.0,3.0>"));
        assert_eq!(Ok(Point::new(1.0, 2.0, 3.0)), point("< 1.0 , 2.0 , 3.0 >"));
    }

    #[test]
    pub fn parse_direction() {
        assert_eq!(Ok(Direction::new(1.0, 2.0, 3.0)), direction("<1.0,2.0,3.0>"));
        assert_eq!(Ok(Direction::new(1.0, 2.0, 3.0)), direction("< 1.0 , 2.0 , 3.0 >"));
    }

    #[test]
    fn parse_color() {
        assert_eq!(Ok(Color::white()), color("color white"));
        assert_eq!(Ok(Color::black()), color("color black"));
        assert_eq!(Ok(Color::new(0.3, 0.6, 0.9)), color("color rgb <0.3,0.6,0.9>"));
        assert_eq!(Ok(Color::new(0.3, 0.6, 0.9)), color("color rgb < 0.3 , 0.6 , 0.9 >"));
    }

    #[test]
    fn parse_camera() {
        assert_eq!(
            Ok(Camera::new(Point::new(1.0, 2.0, 3.0), 60.0).look_at(Point::new(4.0, 5.0, 6.0))),
            camera("camera { location <1.0, 2.0, 3.0> look_at <4.0, 5.0, 6.0> }")
        )
    }

    #[test]
    fn parse_point_light() {
        assert_eq!(
            Ok(Light::Point { color: Color::white(), intensity: 1234.0, origin: Point::new(1.0, 2.0, 3.0) }),
            point_light("point_light { <1.0, 2.0, 3.0> color white intensity 1234.0 }")
        )
    }

    #[test]
    fn parse_object() {
        assert_eq!(
            Ok(Object::new(
                "object",
                Shape::Sphere(Sphere::new(4.0)),
                vec![
                    (0.8, Shader::DiffuseSpecular {
                        albedo: DEFAULT_ALBEDO,
                        texture: Texture::Solid(Color::white()),
                        roughness: 0.0,
                        highlight: 0.0,
                    }),
                    (0.2, Shader::DiffuseSpecular {
                        albedo: DEFAULT_ALBEDO,
                        texture: Texture::Solid(Color::black()),
                        roughness: 10.0,
                        highlight: 20.0,
                    }),
                ]
            ).transform(Matrix44f::translation(Direction::new(1.0, 2.0, 3.0)))),
            object(r#"object {
                location <1.0, 2.0, 3.0>
                sphere { radius 4.0 }
                shaders [
                    0.8 diffuse_specular { texture { solid color white } roughness 0.0 highlight 0.0 },
                    0.2 diffuse_specular { texture { solid color black } roughness 10.0 highlight 20.0 }
                ]
            }"#)
        )
    }

    #[test]
    fn parse_scene() {
        let text = r#"
        camera {
            location <1.0, 2.0, 3.0>
            look_at <4.0, 5.0, 6.0>
        }

        point_light {
            <1.0, 2.0, 3.0>
            color white
            intensity 3000.0
        }

        point_light {
            <4.0, 5.0, 6.0>
            color red
            intensity 4000.0
        }

        object {
            location <1.0, 2.0, 3.0>
            sphere { radius 4.0 }
            shaders [
                0.8 diffuse_specular { texture { solid color white } roughness 0.0 highlight 0.0 },
                0.2 diffuse_specular { texture { solid color black } roughness 10.0 highlight 20.0 }
            ]
        }

        object {
            location <4.0, 5.0, 6.0>
            sphere { radius 7.0 }
            shaders [
                0.3 diffuse_specular { texture { solid color white } roughness 0.0 highlight 0.0 },
                0.7 diffuse_specular { texture { solid color black } roughness 10.0 highlight 20.0 }
            ]
        }
        "#;

        let scene = Scene {
            camera: Camera::new(Point::new(1.0, 2.0, 3.0), 60.0).look_at(Point::new(4.0, 5.0, 6.0)),
            lights: vec![
                Light::Point { color: Color::white(), intensity: 3000.0, origin: Point::new(1.0, 2.0, 3.0) },
                Light::Point { color: Color::red(), intensity: 4000.0, origin: Point::new(4.0, 5.0, 6.0) },
            ],
            objects: vec![
                Object::new(
                    "object",
                    Shape::Sphere(Sphere::new(4.0)),
                    vec![
                        (0.8, Shader::DiffuseSpecular {
                            albedo: DEFAULT_ALBEDO,
                            texture: Texture::Solid(Color::white()),
                            roughness: 0.0,
                            highlight: 0.0,
                        }),
                        (0.2, Shader::DiffuseSpecular {
                            albedo: DEFAULT_ALBEDO,
                            texture: Texture::Solid(Color::black()),
                            roughness: 10.0,
                            highlight: 20.0,
                        }),
                    ]
                ).transform(Matrix44f::translation(Direction::new(1.0, 2.0, 3.0))),
                Object::new(
                    "object",
                    Shape::Sphere(Sphere::new(7.0)),
                    vec![
                        (0.3, Shader::DiffuseSpecular {
                            albedo: DEFAULT_ALBEDO,
                            texture: Texture::Solid(Color::white()),
                            roughness: 0.0,
                            highlight: 0.0,
                        }),
                        (0.7, Shader::DiffuseSpecular {
                            albedo: DEFAULT_ALBEDO,
                            texture: Texture::Solid(Color::black()),
                            roughness: 10.0,
                            highlight: 20.0,
                        }),
                    ]
                ).transform(Matrix44f::translation(Direction::new(4.0, 5.0, 6.0))),
            ],
        };

        assert_eq!(Ok(scene), parse(text))
    }
}
