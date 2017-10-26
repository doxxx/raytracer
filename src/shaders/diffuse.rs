use shaders::Shader;

use color::Color;
use direction::{Direction, Dot};
use photon::PhotonData;
use system::{RenderContext, Ray, SurfaceInfo};
use texture::{ColorSource, Texture};

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Clone)]
pub struct Diffuse {
    pub albedo: f64,
    pub texture: Texture,
    pub diffuse_factor: f64,
    pub specular_factor: f64,
    pub highlight: f64,
}

impl Diffuse {
    fn lambertian(&self, dir: Direction, si: &SurfaceInfo) -> f64 {
        si.n.dot(-dir).max(0.0) * self.albedo
    }

    fn phong(&self, dir: Direction, si: &SurfaceInfo) -> f64 {
        let r = dir.reflect(si.n);
        r.dot(-dir).max(0.0).powf(self.highlight)
    }
}

impl Shader for Diffuse {
    fn shade_point(&self, context: &RenderContext, si: &SurfaceInfo) -> Color {
        let mut diffuse = Color::black();
        let mut specular = Color::black();

        let surface_color = self.texture.color_at_uv(si.uv);

        for light in &context.scene.lights {
            let (dir, intensity, distance) = light.illuminate(si.point);
            let shadow_ray = Ray::shadow(si.point + si.n * context.options.bias, -dir, 0);

            if shadow_ray.trace(&context.scene.objects, distance).is_none() {
                diffuse += surface_color * intensity * self.lambertian(dir, si);
                specular += intensity * self.phong(dir, si);
            }
        }

        let indirect_illumination = match context.photon_map {
            Some(ref m) => {
                let photons: Vec<PhotonData> = m.find_nearest_n(si.point, 50).iter().map(|d| d.value).collect();
                photons.iter()
                    .map(|pd| surface_color * pd.power * self.lambertian(pd.incident, si))
                    .fold(Color::black(), |a,b| a + b)
            },
            None => Color::black()
        };

//        if indirect_illumination != Color::black() {
//            println!("indirect_illumination: {:?}", indirect_illumination);
//        }

        diffuse * self.diffuse_factor + specular * self.specular_factor + indirect_illumination
//        indirect_illumination

    }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}
