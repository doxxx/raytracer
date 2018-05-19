use color::Color;
use system::Ray;
use system::{RenderContext,SurfaceInfo};

pub trait Material: Send + Sync {
    fn interact(&self, context: &RenderContext, si: &SurfaceInfo) -> SurfaceInteraction;
    // fn color(&self, context: &RenderContext, si: &SurfaceInfo) -> Color;
    // fn has_transparency(&self) -> bool { false }
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct SurfaceInteraction {
    pub absorbed: bool,
    pub emittance: Color,
    pub attenuation: Color,
    pub scattered: Ray,
}

mod lambertian;
mod metal;
mod dielectric;
mod diffuse_light;

pub use self::lambertian::Lambertian;
pub use self::metal::Metal;
pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
