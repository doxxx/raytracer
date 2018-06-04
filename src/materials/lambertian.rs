use color::Color;
use direction::Direction;
use materials::MaterialInteraction;
use system::{Ray, RayHit, RenderContext};
use texture::{ColorSource, Texture};

use materials::Material;

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
    fn interact(&self, context: &RenderContext, hit: &RayHit) -> MaterialInteraction {
        let scattered_origin = hit.p + hit.n * context.options.bias;
        let target = hit.p + hit.n + Direction::uniform_sphere_distribution();
        let scattered_dir = (target - hit.p).normalize();

        MaterialInteraction::Scattered {
            albedo: self.texture.color_at_uv(hit.uv),
            dir: Ray::primary(scattered_origin, scattered_dir, hit.incident.depth + 1),
        }
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
