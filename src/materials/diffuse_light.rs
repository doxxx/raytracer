use color::Color;
use direction::Direction;
use materials::SurfaceInteraction;
use point::Point;
use system::{Ray, RenderContext, SurfaceInfo};
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
    fn interact(&self, _context: &RenderContext, si: &SurfaceInfo) -> SurfaceInteraction {
        SurfaceInteraction {
            absorbed: true,
            emittance: self.intensity * self.texture.color_at_uv(si.uv),
            attenuation: Color::black(),
            scattered: Ray::primary(Point::zero(), Direction::zero(), 0),
        }
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
