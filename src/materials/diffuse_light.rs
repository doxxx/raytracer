use color::Color;
use direction::Direction;
use materials::SurfaceInteraction;
use point::Point;
use system::{Ray, RenderContext, SurfaceInfo};
use texture::{ColorSource, Texture};

use materials::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    texture: Texture,
}

impl DiffuseLight {
    pub fn new(texture: Texture) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn interact(&self, _context: &RenderContext, si: &SurfaceInfo) -> SurfaceInteraction {
        SurfaceInteraction {
            absorbed: true,
            emittance: self.texture.color_at_uv(si.uv),
            attenuation: Color::black(),
            scattered: Ray::primary(Point::zero(), Direction::zero(), 0),
        }
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
