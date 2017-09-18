use std::mem;

use color::Color;
use direction::{Direction, Dot};
use object::Object;
use point::Point;
use system::{RenderContext, Ray, SurfaceInfo};
use texture::{ColorSource,Texture};

pub const DEFAULT_ALBEDO: f64 = 0.18;

pub const IOR_WATER: f64 = 1.3;
pub const IOR_GLASS: f64 = 1.5;
pub const IOR_DIAMOND: f64 = 1.8;

#[derive(Debug, Clone, PartialEq)]
pub enum Shader {
    DiffuseSpecular {
        albedo: f64,
        texture: Texture,
        roughness: f64,
        highlight: f64,
    },
    Reflection,
    Transparency {
        ior: f64,
    }
}

impl Shader {
    pub fn shade_point(&self, context: &RenderContext, depth: u16, view: Direction, object: &Object, si: &SurfaceInfo) -> Color {
        match self {
            &Shader::DiffuseSpecular { albedo, ref texture, roughness, highlight } => {
                let mut c1 = Color::black();
                let mut c2 = Color::black();
                for light in &context.scene.lights {
                    let (dir, intensity, distance) = light.illuminate(si.point);
                    let shadow_ray = Ray::shadow(si.point + si.n * context.options.bias, -dir);

                    if shadow_ray.trace(&context.scene.objects, distance).is_none() {
                        let dot = si.n.dot(-dir).max(0.0);
                        if dot > 0.0 {
                            c1 += texture.color_at_uv(si.uv) * albedo * intensity * dot;
                        }
                        let r = reflect(dir, si.n);
                        c2 += intensity * r.dot(-dir).max(0.0).powf(highlight); // todo: specular color
                    }
                }

                c1 + c2 * roughness
            },
            &Shader::Reflection => {
                let reflection_ray = Ray::primary(
                    si.point + si.n * context.options.bias,
                    reflect(view, si.n).normalize(),
                );
                reflection_ray.cast(context, depth + 1)
            },
            &Shader::Transparency { ior } => {
                let mut refraction_color = Color::black();
                let kr = fresnel(view, si.n, ior);
                let outside = view.dot(si.n) < 0.0;
                let bias = si.n * context.options.bias;
                if kr < 1.0 {
                    let refraction_ray = Ray::primary(
                        if outside {
                            si.point - bias
                        } else {
                            si.point + bias
                        },
                        refract(view, si.n, ior).normalize(),
                    );
                    refraction_color = refraction_ray.cast(context, depth + 1);
                }
                let reflection_ray = Ray::primary(
                    if outside {
                        si.point + bias
                    } else {
                        si.point - bias
                    },
                    reflect(view, si.n).normalize(),
                );
                let reflection_color = reflection_ray.cast(context, depth + 1);
                reflection_color * kr * 0.8 + refraction_color * (1.0 - kr)
            }
        }
    }
}

fn clamp(lo: f64, hi: f64, val: f64) -> f64 {
    lo.max(hi.min(val))
}

fn reflect(incident: Direction, normal: Direction) -> Direction {
    incident - normal * 2.0 * incident.dot(normal)
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

/// incident, normal, index of reflection -> reflection
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

