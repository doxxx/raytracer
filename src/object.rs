use material::Color;
use material::Material;
use shapes::Shape;
use vector::Vector3f;

pub struct Object {
    pub shape: Box<Shape>,
    pub material: Box<Material>,
}

impl Object {
    pub fn new(shape: Box<Shape>, material: Box<Material>) -> Object {
        Object {
            shape: shape,
            material: material,
        }
    }

    pub fn color(&self, point: Vector3f, incidence: Vector3f) -> Color {
        let (normal, texture_coords) = self.shape.surface_data(point);
        self.material.color(texture_coords) * f64::max(0.0, normal.dot(-incidence))
    }
}
