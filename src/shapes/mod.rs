use system::Intersectable;

pub mod bounding_box;
pub mod composite;
pub mod mesh;
pub mod plane;
pub mod sphere;

use matrix::Matrix44f;
use shapes::bounding_box::BoundingBox;

pub trait Shape: Intersectable + Send + Sync {
    fn bounding_box(&self, m: Matrix44f) -> BoundingBox;
}
