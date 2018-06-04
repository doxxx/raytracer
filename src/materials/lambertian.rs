use std::f64;

use color::Color;
use direction::*;
use materials::*;
use system::{Ray, RayHit, RenderContext};
use texture::{ColorSource, Texture};

#[derive(Clone)]
pub struct Lambertian {
    texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Lambertian {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn kind(&self) -> MaterialKind {
        MaterialKind::NonEmitting
    }
    
    fn interact(&self, context: &RenderContext, hit: &RayHit) -> MaterialInteraction {
        let uvw = OrthoNormalBase::new(hit.n);
        let scattered_dir = uvw.local(Direction::uniform_sphere_distribution()).normalize();
        let scattered_origin = hit.p + hit.n * context.options.bias;

        MaterialInteraction::Scattered {
            albedo: self.texture.color_at_uv(hit.uv),
            dir: Ray::primary(scattered_origin, scattered_dir, hit.incident.depth + 1),
            pdf: uvw.w.dot(scattered_dir) / f64::consts::PI,
        }
    }

    fn scattering_pdf(&self, context: &RenderContext, hit: &RayHit, scattered: &Ray) -> f64 {
        let mut cosine = hit.n.dot(scattered.direction);
        if cosine < 0.0 {
            cosine = 0.0;
        }
        cosine / f64::consts::PI
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Copy, Clone)]
struct OrthoNormalBase {
    u: Direction,
    v: Direction,
    w: Direction,
}

impl OrthoNormalBase {
    fn new(n: Direction) -> OrthoNormalBase {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Direction::new(0.0, 1.0, 0.0)
        } else {
            Direction::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        OrthoNormalBase { u, v, w }
    }

    fn local(&self, a: Direction) -> Direction {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
