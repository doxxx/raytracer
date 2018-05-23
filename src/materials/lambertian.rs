use color::Color;
use direction::Direction;
use materials::SurfaceInteraction;
use system::{Ray, RenderContext, SurfaceInfo};
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
    fn interact(&self, context: &RenderContext, si: &SurfaceInfo) -> SurfaceInteraction {
        let scattered_origin = si.point + si.n * context.options.bias;
        let target = si.point + si.n + Direction::uniform_sphere_distribution();
        let scattered_dir = (target - si.point).normalize();

        SurfaceInteraction {
            absorbed: false,
            emittance: Color::black(),
            attenuation: self.texture.color_at_uv(si.uv),
            scattered: Ray::primary(scattered_origin, scattered_dir, si.incident.depth + 1),
        }
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
