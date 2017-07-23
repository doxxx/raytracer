use material::Material;
use shapes::Shape;

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
}
