use lights::Light;
use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::{Vector2f, Vector3f};

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub texture: Box<Texture>,
    pub albedo: Vector3f,
}

impl Object {
    pub fn new(name: &'static str, shape: Box<Shape>, texture: Box<Texture>, albedo: Option<Color>) -> Object {
        Object {
            name: name,
            shape: shape,
            texture: texture,
            albedo: albedo.unwrap_or(Vector3f(0.18, 0.18, 0.18)),
        }
    }
}
