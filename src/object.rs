use shapes::Shape;
use system::Color;
use texture::Texture;
use vector::Vector3f;

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

    pub fn color(&self, point: Vector3f, incidence: Vector3f) -> Color {
        let (normal, texture_coords) = self.shape.surface_data(point);
        let color = self.texture.color(texture_coords);
        let facing_ratio = f64::max(0.0, normal.dot(-incidence));
        color * facing_ratio
    }
}
