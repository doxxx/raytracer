use crate::color::Color;
use crate::direction::Direction;
use crate::materials::ScatteredRay;
use crate::system::{RayHit, RenderContext};
use crate::texture::{ColorSource, Texture};

use crate::materials::Material;

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

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
