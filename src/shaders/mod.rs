use color::Color;
use direction::Direction;
use object::Object;
use system::{RenderContext, SurfaceInfo};

pub trait Shader: Send + Sync {
    fn shade_point(&self, context: &RenderContext, depth: u16, view: Direction, object: &Object, si: &SurfaceInfo) -> Color;
    fn has_transparency(&self) -> bool { false }
    fn box_clone(&self) -> Box<Shader>;
}

impl Clone for Box<Shader> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct ShaderApplication(pub f64, pub Box<Shader>);

impl ShaderApplication {
    pub fn has_transparency(&self) -> bool {
        self.1.has_transparency()
    }
}

pub mod diffuse;
pub mod reflection;
pub mod transparency;


