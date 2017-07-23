use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::{Vector2f, Vector3f};

#[derive(Debug)]
pub struct Object {
    pub shape: Box<Shape>,
    pub texture: Box<Texture>,
}

impl Object {
    pub fn new(shape: Box<Shape>, texture: Box<Texture>) -> Object {
        Object {
            shape: shape,
            texture: texture,
        }
    }

    pub fn color(&self, incidence: Vector3f, normal: Vector3f, uv: Vector2f) -> Color {
        let color = self.texture.color(uv);
        let facing_ratio = f64::max(0.0, normal.dot(-incidence));
        color * facing_ratio
    }
}
