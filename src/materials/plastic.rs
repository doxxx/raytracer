use color::Color;
use shaders::Shader;
use shaders::diffuse::{Diffuse,DEFAULT_ALBEDO};
use shaders::reflection::Reflection;
use system::{RenderContext,SurfaceInfo};
use texture::Texture;

use materials::Material;

#[derive(Clone)]
pub struct Plastic {
    diffuse: Diffuse,
    reflection: Reflection,
}

impl Plastic {
    pub fn new(texture: Texture) -> Plastic {
        Plastic {
            diffuse: Diffuse {
                albedo: DEFAULT_ALBEDO,
                texture,
                roughness: 0.2,
                highlight: 50.0,
            },
            reflection: Reflection {},
        }
    }
}

impl Material for Plastic {
    fn color(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        let dc = self.diffuse.shade_point(context, si);
        let rc = self.reflection.shade_point(context, si);
        (0.8 * dc) + (0.2 * rc)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
