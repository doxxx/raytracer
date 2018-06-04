use color::Color;
use direction::{Direction, Dot};
use materials::MaterialInteraction;
use system::{Ray, RayHit, RenderContext};
use texture::{ColorSource, Texture};

use materials::Material;

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
    fn interact(&self, context: &RenderContext, hit: &RayHit) -> MaterialInteraction {
        let reflected = hit.incident.direction.reflect(hit.n).normalize();
        let fuzz = self.fuzz * Direction::uniform_sphere_distribution();
        let scattered_origin = hit.p + hit.n * context.options.bias;
        let scattered_dir = (reflected + fuzz).normalize();

        if scattered_dir.dot(hit.n) < 0.0 {
            MaterialInteraction::Absorbed
        } else {
            MaterialInteraction::Scattered {
                albedo: self.texture.color_at_uv(hit.uv),
                dir: Ray::primary(scattered_origin, scattered_dir, hit.incident.depth + 1),
            }
        }
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
