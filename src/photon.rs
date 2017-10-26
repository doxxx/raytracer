use std::f64;

use rand;

use color::Color;
use direction::{Direction, Dot};
use kdtree;
use lights::Light;
use sdl::Scene;
use system::{Ray, RayHit, SurfaceInfo, Options};

const DIFFUSE_REFLECTION_PB: f64 = 0.5;
const SPECULAR_REFLECTION_PB: f64 = 0.2;

#[derive(Clone, Copy)]
pub struct PhotonData {
    pub power: Color,
    pub incident: Direction,
}

pub type PhotonMap = Box<kdtree::Tree<PhotonData>>;
pub type PhotonNode = kdtree::Data<PhotonData>;
pub type PhotonReflection = (Direction, Color);

pub fn trace_photon(options: Options, scene: &Scene, light: &Box<Light>, ray: Ray, power: Color, photons: &mut Vec<PhotonNode>) {
    if ray.depth > options.max_depth {
        return;
    }

    if let Some(hit) = ray.trace(&scene.objects, f64::MAX) {
        let ip = hit.i.point(&ray);

        if ray.depth > 0 {
            photons.push(kdtree::Data::new(ip, PhotonData {
                power,
                incident: ray.direction,
            }));
        }

        if let Some((reflected_dir, reflected_power)) = reflect_photon(&ray, &hit, power) {
            let reflected_ray = Ray::primary(
                ip + options.bias * hit.i.n,
                reflected_dir,
                ray.depth + 1
            );

            trace_photon(options, scene, light, reflected_ray, reflected_power, photons)
        }
    }
}

fn reflect_photon(ray: &Ray, hit: &RayHit, power: Color) -> Option<PhotonReflection> {
    let rr: f64 = rand::random();
    if rr < DIFFUSE_REFLECTION_PB {
        // diffuse reflection
        let ip = hit.i.point(ray);
        let si = SurfaceInfo {
            incident: *ray,
            point: ip,
            n: hit.i.n,
            uv: hit.i.uv,
        };
        let surface_color = hit.object.material.surface_color(&si);
        let reflected_power = surface_color * power;

        // random reflection (diffuse)
        let mut reflected_dir = Direction::uniform_sphere_distribution();
        if reflected_dir.dot(hit.i.n) < 0.0 {
            // if it's not in the hemisphere oriented around the surface normal, invert
            reflected_dir *= -1.0;
        }

        return Some((reflected_dir, reflected_power));
    } else if rr < DIFFUSE_REFLECTION_PB + SPECULAR_REFLECTION_PB {
        // specular reflection
        let surface_color = Color::white();
        let reflected_power = surface_color * power;

        // perfect reflection
        let reflected_dir = ray.direction.reflect(hit.i.n);

        return Some((reflected_dir, reflected_power));
    }

    // absorption
    None
}
