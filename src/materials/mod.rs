use crate::color::Color;
use crate::direction::Direction;
use crate::point::Point;
use crate::system::{RayHit, RenderContext};

pub trait Material: Send + Sync {
    fn scatter(&self, context: &RenderContext, hit: &RayHit) -> Option<ScatteredRay>;
    fn emit(&self, context: &RenderContext, hit: &RayHit) -> Color;
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct ScatteredRay {
    pub attenuation: Color,
    pub origin: Point,
    pub direction: Direction,
}

mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
pub use self::isotropic::Isotropic;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
