use color::Color;
use shaders::Shader;
use shaders::transparency::{Transparency,IOR_GLASS};
use system::{RenderContext,SurfaceInfo};

use materials::Material;

#[derive(Clone)]
pub struct Glass {
    transparency: Transparency
}

impl Glass {
    pub fn new() -> Glass {
        Glass {
            transparency: Transparency { ior: IOR_GLASS }
        }
    }
}

impl Material for Glass {
    fn color(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        self.transparency.shade_point(context, si)
    }

    fn has_transparency(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }

    fn surface_color(&self, si: &SurfaceInfo) -> Color {
        Color::white()
    }
}
