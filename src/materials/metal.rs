use color::Color;
use direction::{Direction, Dot};
use materials::SurfaceInteraction;
use system::{Ray, RenderContext, SurfaceInfo};
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
    fn interact(&self, context: &RenderContext, si: &SurfaceInfo) -> SurfaceInteraction {
        let reflected = si.incident.direction.reflect(si.n).normalize();
        let fuzz = self.fuzz * Direction::uniform_sphere_distribution();
        let scattered_origin = si.point + si.n * context.options.bias;
        let scattered_dir = (reflected + fuzz).normalize();

        SurfaceInteraction {
            absorbed: scattered_dir.dot(si.n) < 0.0,
            emittance: Color::black(),
            attenuation: self.texture.color_at_uv(si.uv),
            scattered: Ray::primary(scattered_origin, scattered_dir, si.incident.depth + 1),
        }
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
