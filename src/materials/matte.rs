use color::Color;
use shaders::Shader;
use shaders::diffuse::{Diffuse,DEFAULT_ALBEDO};
use system::{RenderContext,SurfaceInfo};
use texture::Texture;

use materials::Material;

#[derive(Clone)]
pub struct Matte {
    diffuse: Diffuse,
}

impl Matte {
    pub fn new(texture: Texture) -> Matte {
        Matte {
            diffuse: Diffuse {
                albedo: DEFAULT_ALBEDO,
                texture,
                roughness: 0.0,
                highlight: 0.0,
            }
        }
    }
}

impl Material for Matte {
    fn color(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        self.diffuse.shade_point(context, si)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
