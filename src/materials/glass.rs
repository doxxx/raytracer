use color::Color;
use direction::Direction;
use object::Object;
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
    fn color(&self, context: &RenderContext, depth: u16, view: Direction, object: &Object, si: &SurfaceInfo) -> Color {
        self.transparency.shade_point(context, depth, view, object, si)
    }

    fn has_transparency(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}
