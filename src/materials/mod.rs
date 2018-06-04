use color::Color;
use system::Ray;
use system::{RayHit, RenderContext};

#[derive(Debug, PartialEq)]
pub enum MaterialKind {
    NonEmitting,
    Emitting,
}

pub trait Material: Send + Sync {
    fn kind(&self) -> MaterialKind;
    fn interact(&self, context: &RenderContext, hit: &RayHit) -> MaterialInteraction;
    fn scattering_pdf(&self, context: &RenderContext, hit: &RayHit, scattered: &Ray) -> f64;
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
    Scattered { albedo: Color, dir: Ray, pdf: f64 },
}

mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
