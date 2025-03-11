use std::mem;

use rand;
use rand::Rng;

use crate::color::Color;
use crate::direction::{Direction, Dot};
use crate::materials::ScatteredRay;
use crate::system::{RenderContext, RayHit};

use crate::materials::Material;

#[derive(Clone)]
pub struct Dielectric {
    ior: f64,
    fuzz: f64,
}

impl Dielectric {
    pub fn new(ior: f64, fuzz: f64) -> Dielectric {
        Dielectric { ior, fuzz }
    }
}

impl Material for Dielectric {
    fn scatter(&self, context: &RenderContext, hit: &RayHit) -> Option<ScatteredRay> {
        let p = hit.point();
        let outside = hit.incident.direction.dot(hit.n) < 0.0;
        let bias = hit.n * context.options.bias;

        let kr = fresnel(hit.incident.direction, hit.n, self.ior);
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < kr { 
            // reflection
            let reflected = hit.incident.direction.reflect(hit.n);
            let fuzz = self.fuzz * Direction::uniform_sphere_distribution();
            let scattered = (reflected + fuzz).normalize();
            Some(ScatteredRay {
                attenuation: Color::white(),
                origin: if outside { p + bias } else { p - bias },
                direction: scattered,
            })
        } else { 
            // refraction
            let refracted = refract(hit.incident.direction, hit.n, self.ior);
            let fuzz = self.fuzz * Direction::uniform_sphere_distribution();
            let scattered = (refracted + fuzz).normalize();
            Some(ScatteredRay {
                attenuation: Color::white(),
                origin: if outside { p - bias } else { p + bias },
                direction: scattered,
            })
        }
    }

    fn emit(&self, _context: &RenderContext, _hit: &RayHit) -> Color {
        Color::black()
    }

    fn box_clone(&self) -> Box<dyn Material> {
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
