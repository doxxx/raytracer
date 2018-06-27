use crate::color::Color;
use crate::direction::Direction;
use crate::materials::ScatteredRay;
use crate::system::{RayHit, RenderContext};
use crate::texture::{ColorSource, Texture};

use crate::materials::Material;

#[derive(Clone)]
pub struct Lambertian {
    texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Lambertian {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, context: &RenderContext, hit: &RayHit) -> Option<ScatteredRay> {
        let p = hit.point();
        let scattered_origin = p + hit.n * context.options.bias;
        let target = p + hit.n + Direction::uniform_sphere_distribution();
        let scattered_dir = (target - p).normalize();

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
