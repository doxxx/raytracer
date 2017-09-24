use shaders::Shader;

use color::Color;
use direction::Dot;
use system::{RenderContext, Ray, SurfaceInfo};
use texture::{ColorSource,Texture};

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Clone)]
pub struct Diffuse {
    pub albedo: f64,
    pub texture: Texture,
    pub roughness: f64,
    pub highlight: f64,
}

impl Shader for Diffuse {
    fn shade_point(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        let mut c1 = Color::black();
        let mut c2 = Color::black();

        for light in &context.scene.lights {
            let (dir, intensity, distance) = light.illuminate(si.point);
            let shadow_ray = Ray::shadow(si.point + si.n * context.options.bias, -dir, 0);

            if shadow_ray.trace(&context.scene.objects, distance).is_none() {
                let dot = si.n.dot(-dir).max(0.0);
                if dot > 0.0 {
                    c1 += self.texture.color_at_uv(si.uv) * self.albedo * intensity * dot;
                }
                let r = dir.reflect(si.n);
                c2 += intensity * r.dot(-dir).max(0.0).powf(self.highlight); // todo: specular color
            }
        }

        c1 + c2 * self.roughness
    }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}
