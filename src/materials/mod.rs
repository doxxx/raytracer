use color::Color;
use system::Ray;
use system::{RayHit, RenderContext};

pub trait Material: Send + Sync {
    fn interact(&self, context: &RenderContext, hit: &RayHit) -> MaterialInteraction;
    fn emit(&self, context: &RenderContext, hit: &RayHit) -> Color;
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub enum MaterialInteraction {
    Absorbed,
    Scattered { albedo: Color, dir: Ray },
}

mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
