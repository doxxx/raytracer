use shaders::Shader;

use color::Color;
use direction::Direction;
use system::{RenderContext, Ray, SurfaceInfo};

#[derive(Clone)]
pub struct Reflection {}

impl Shader for Reflection {
    fn shade_point(&self, context: &RenderContext, depth: u16, view: Direction, si: &SurfaceInfo) -> Color {
        let reflection_ray = Ray::primary(
            si.point + si.n * context.options.bias,
            view.reflect(si.n).normalize(),
        );
        reflection_ray.cast(context, depth + 1)
    }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}
