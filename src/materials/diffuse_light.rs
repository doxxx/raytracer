use color::Color;
use materials::MaterialInteraction;
use system::{RayHit, RenderContext};
use texture::{ColorSource, Texture};

use materials::Material;

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
    fn interact(&self, _context: &RenderContext, _hit: &RayHit) -> MaterialInteraction {
        MaterialInteraction::Absorbed
    }

    fn emit(&self, _context: &RenderContext, hit: &RayHit) -> Color {
        self.intensity * self.texture.color_at_uv(hit.uv)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
