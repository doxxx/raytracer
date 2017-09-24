use color::Color;
use direction::Direction;
use system::{RenderContext,SurfaceInfo};

pub trait Material: Send + Sync {
    fn color(&self, context: &RenderContext, depth: u16, view: Direction, si: &SurfaceInfo) -> Color;
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
