use color::Color;
use direction::Direction;
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
    fn color(&self, context: &RenderContext, depth: u16, view: Direction, si: &SurfaceInfo) -> Color {
        self.diffuse.shade_point(context, depth, view, si)
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
