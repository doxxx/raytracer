use std::fmt::Debug;
use std::f64::consts::PI;

use system::Color;
use vector::Vector3f;


pub trait Light: Debug {
    fn calculate_color(&self, albedo: Color, surface_normal: Vector3f) -> Color;
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
    fn calculate_color(&self, albedo: Color, surface_normal: Vector3f) -> Color {
        albedo / PI * self.intensity * self.color * surface_normal.dot(-self.direction).max(0.0)
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
