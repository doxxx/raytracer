use color::Color;
use direction::Direction;
use system::{RenderContext, SurfaceInfo};

pub trait Shader: Send + Sync {
    fn shade_point(&self, context: &RenderContext, depth: u16, view: Direction, si: &SurfaceInfo) -> Color;
    fn has_transparency(&self) -> bool { false }
    fn box_clone(&self) -> Box<Shader>;
}

impl Clone for Box<Shader> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub mod diffuse;
pub mod reflection;
pub mod transparency;


