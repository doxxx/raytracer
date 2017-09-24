use system::Intersectable;

pub mod bounding_box;
pub mod composite;
pub mod mesh;
pub mod plane;
pub mod sphere;

pub trait Shape: Intersectable + Send + Sync {}
