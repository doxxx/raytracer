use lights::Light;
use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::{Vector2f, Vector3f};

#[derive(Debug)]
pub struct Object {
    pub shape: Box<Shape>,
    pub texture: Box<Texture>,
    pub albedo: Vector3f,
}

impl Object {
    pub fn new(shape: Box<Shape>, texture: Box<Texture>, albedo: Option<Color>) -> Object {
        Object {
            shape: shape,
            texture: texture,
            albedo: albedo.unwrap_or(Vector3f(0.18, 0.18, 0.18)),
        }
    }

    pub fn color(
        &self,
        incident: Vector3f,
        normal: Vector3f,
        uv: Vector2f,
        light: &Box<Light>,
    ) -> Color {
        // let color = self.texture.color(uv);
        // let facing_ratio = f64::max(0.0, normal.dot(-incident));
        // color * facing_ratio
        light.calculate_color(self.albedo, normal)
    }
}
