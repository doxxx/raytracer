use color::Color;
use system::{RenderContext,SurfaceInfo};

pub trait Material: Send + Sync {
    fn color(&self, context: &RenderContext, si: &SurfaceInfo) -> Color;
    fn surface_color(&self, si: &SurfaceInfo) -> Color;
    fn has_transparency(&self) -> bool { false }
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub mod matte;
pub mod plastic;
pub mod glass;
