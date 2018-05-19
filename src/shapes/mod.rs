use system::Intersectable;

pub mod bounding_box;
pub mod composite;
pub mod mesh;
pub mod plane;
pub mod sphere;

pub use self::bounding_box::*;
pub use self::composite::*;
pub use self::mesh::*;
pub use self::plane::*;
pub use self::sphere::*;

pub trait Shape: Intersectable + Send + Sync {}
