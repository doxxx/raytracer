use shaders::Shader;

use color::Color;
use system::{RenderContext, Ray, SurfaceInfo};

#[derive(Clone)]
pub struct Reflection {}

impl Shader for Reflection {
    fn shade_point(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        let reflection_ray = Ray::primary(
            si.point + si.n * context.options.bias,
            si.incident.direction.reflect(si.n).normalize(),
            si.incident.depth + 1,
        );
        reflection_ray.cast(context)
    }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}
