use std::f64::consts::PI;
use std::fmt::Debug;

use system::Color;
use vector::Vector3f;


pub trait Light: Debug {
    fn get_direction_from_point(&self, point: Vector3f) -> Vector3f;
    fn get_surface_color(&self, albedo: Color, surface_point: Vector3f, surface_normal: Vector3f) -> Color;
}


#[derive(Debug)]
pub struct DistantLight {
    color: Color,
    intensity: f64,
    direction: Vector3f,
}

impl DistantLight {
    pub fn new(color: Color, intensity: f64, direction: Vector3f) -> DistantLight {
        DistantLight {
            color: color,
            intensity: intensity,
            direction: direction,
        }
    }
}

impl Light for DistantLight {
    fn get_direction_from_point(&self, point: Vector3f) -> Vector3f {
        -self.direction
    }

    fn get_surface_color(&self, albedo: Color, surface_point: Vector3f, surface_normal: Vector3f) -> Color {
        let surface_light_dot = surface_normal.dot(-self.direction);
        albedo / PI * self.intensity * self.color * surface_light_dot.max(0.0)
    }
}



// #[derive(Debug)]
// pub struct PointLight {
//     color: Color,
//     intensity: f64,
//     origin: Vector3f,
//     direction: Vector3f,
// }

// impl PointLight {
//     pub fn new(color: Color, intensity: f64, origin: Vector3f, direction: Vector3f) -> PointLight {
//         PointLight {
//             color: color,
//             intensity: intensity,
//             origin: origin,
//             direction: direction,
//         }
//     }
// }

// impl Light for PointLight {}
