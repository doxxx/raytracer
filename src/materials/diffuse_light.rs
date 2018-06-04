use color::Color;
use direction::*;
use materials::*;
use system::{Ray, RayHit, RenderContext};
use texture::{ColorSource, Texture};

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
    fn kind(&self) -> MaterialKind {
        MaterialKind::Emitting
    }
    
    fn interact(&self, _context: &RenderContext, _hit: &RayHit) -> MaterialInteraction {
        MaterialInteraction::Absorbed
    }

    fn scattering_pdf(&self, context: &RenderContext, hit: &RayHit, scattered: &Ray) -> f64 {
        0.0 // TODO
    }

    fn emit(&self, _context: &RenderContext, hit: &RayHit) -> Color {
        if hit.n.dot(hit.incident.direction) < 0.0 {
            self.intensity * self.texture.color_at_uv(hit.uv)
        } else {
            Color::black()
        }
        
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
