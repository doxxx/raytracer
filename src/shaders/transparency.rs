use std::mem;

use shaders::Shader;

use color::Color;
use direction::{Direction, Dot};
use system::{RenderContext, Ray, SurfaceInfo};

pub const IOR_WATER: f64 = 1.3;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 1.8;

#[derive(Clone)]
pub struct Transparency {
    pub ior: f64,
}

impl Shader for Transparency {
    fn shade_point(&self, context: &RenderContext, depth: u16, view: Direction, si: &SurfaceInfo) -> Color {
        let mut refraction_color = Color::black();
        let kr = fresnel(view, si.n, self.ior);
        let outside = view.dot(si.n) < 0.0;
        let bias = si.n * context.options.bias;
        if kr < 1.0 {
            let refraction_ray = Ray::primary(
                if outside {
                    si.point - bias
                } else {
                    si.point + bias
                },
                refract(view, si.n, self.ior).normalize(),
            );
            refraction_color = refraction_ray.cast(context, depth + 1);
        }
        let reflection_ray = Ray::primary(
            if outside {
                si.point + bias
            } else {
                si.point - bias
            },
            view.reflect(si.n).normalize(),
        );
        let reflection_color = reflection_ray.cast(context, depth + 1);
        reflection_color * kr * 0.8 + refraction_color * (1.0 - kr)
    }

    fn has_transparency(&self) -> bool { true }

    fn box_clone(&self) -> Box<Shader> {
        Box::new(self.clone())
    }
}

fn clamp(lo: f64, hi: f64, val: f64) -> f64 {
    lo.max(hi.min(val))
}

fn refract(incident: Direction, normal: Direction, ior: f64) -> Direction {
    let mut cos_i = clamp(-1.0, 1.0, incident.dot(normal));
    let mut eta_i = 1.0;
    let mut eta_t = ior;
    let mut n = normal;
    if cos_i < 0.0 {
        cos_i = -cos_i;
    } else {
        mem::swap(&mut eta_i, &mut eta_t);
        n = -normal;
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
    if k < 0.0 {
        Direction::zero()
    } else {
        incident * eta + n * (eta * cos_i - k.sqrt())
    }
}

/// incident, normal, index of reflection -> reflection factor
fn fresnel(incident: Direction, normal: Direction, ior: f64) -> f64 {
    let mut cos_i = clamp(-1.0, 1.0, incident.dot(normal));
    let mut eta_i = 1.0;
    let mut eta_t = ior;
    if cos_i > 0.0 {
        mem::swap(&mut eta_i, &mut eta_t);
    }
    let sin_t = eta_i / eta_t * (1.0 - cos_i * cos_i).max(0.0).sqrt();

    if sin_t >= 1.0 {
        // total internal reflection
        1.0
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        cos_i = cos_i.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        (r_s * r_s + r_p * r_p) / 2.0
    }
}

