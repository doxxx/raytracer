use crate::color::Color;
use crate::direction::Direction;
use crate::materials::ScatteredRay;
use crate::system::{RayHit, RenderContext};
use crate::texture::{ColorSource, Texture};

use crate::materials::Material;

#[derive(Clone)]
pub struct Metal {
    fuzz: f64,
    texture: Texture,
}

impl Metal {
    pub fn new(fuzz: f64, texture: Texture) -> Metal {
        Metal { fuzz, texture }
    }
}

impl Material for Metal {
    fn scatter(&self, context: &RenderContext, hit: &RayHit) -> Option<ScatteredRay> {
        let reflected = hit.incident.direction.reflect(hit.n).normalize();
        let fuzz = self.fuzz * Direction::uniform_sphere_distribution();
        let scattered_origin = hit.point() + hit.n * context.options.bias;
        let scattered_dir = (reflected + fuzz).normalize();

        Some(ScatteredRay {
            attenuation: self.texture.color_at_uv(hit.uv),
            origin: scattered_origin,
            direction: scattered_dir,
        })
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
