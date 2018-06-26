use crate::color::Color;
use crate::materials::ScatteredRay;
use crate::system::{RayHit, RenderContext};
use crate::texture::{ColorSource, Texture};

use crate::materials::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    intensity: f64,
    texture: Texture,
}

impl DiffuseLight {
    pub fn new(intensity: f64, texture: Texture) -> DiffuseLight {
        DiffuseLight { intensity, texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _context: &RenderContext, _hit: &RayHit) -> Option<ScatteredRay> {
        None
    }

    fn emit(&self, _context: &RenderContext, hit: &RayHit) -> Color {
        self.intensity * self.texture.color_at_uv(hit.uv)
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
