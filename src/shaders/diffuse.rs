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
    pub diffuse_factor: f64,
    pub specular_factor: f64,
    pub highlight: f64,
}

impl Shader for Diffuse {
    fn shade_point(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        let mut lambertian = Color::black();
        let mut specular = Color::black();

        for light in &context.scene.lights {
            let (dir, intensity, distance) = light.illuminate(si.point);
            let shadow_ray = Ray::shadow(si.point + si.n * context.options.bias, -dir, 0);

            if shadow_ray.trace(&context.scene.objects, distance).is_none() {
                let dot = si.n.dot(-dir).max(0.0);
                if dot > 0.0 {
                    lambertian += self.texture.color_at_uv(si.uv) * self.albedo * intensity * dot;
                }
                let r = dir.reflect(si.n);
                specular += intensity * r.dot(-dir).max(0.0).powf(self.highlight); // todo: specular color
            }
        }

        lambertian * self.diffuse_factor + specular * self.specular_factor
    }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}
