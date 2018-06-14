use color::Color;
use direction::Direction;
use materials::ScatteredRay;
use system::{RayHit, RenderContext};
use texture::{ColorSource, Texture};

use materials::Material;

#[derive(Clone)]
pub struct Isotropic {
    texture: Texture,
}

impl Isotropic {
    pub fn new(texture: Texture) -> Isotropic {
        Isotropic { texture }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _context: &RenderContext, hit: &RayHit) -> Option<ScatteredRay> {
        Some(ScatteredRay {
            attenuation: self.texture.color_at_uv(hit.uv),
            origin: hit.point(),
            direction: Direction::uniform_sphere_distribution(),
        })
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
