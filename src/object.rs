use material::Material;
use shapes::Shape;
use system::Ray;
use vector::Vector3f;

pub const DEFAULT_ALBEDO: f64 = 0.18;

#[derive(Debug)]
pub struct Object {
    pub name: &'static str,
    pub shape: Box<Shape>,
    pub albedo: f64,
    pub material: Material,
    pub bounds: Option<BoundingBox>,
}

impl Object {
    pub fn new(name: &'static str, shape: Box<Shape>, albedo: f64, material: Material) -> Object {
        let bounds = shape.bounding_box();
        Object {
            name: name,
            shape: shape,
            albedo: albedo,
            material: material,
            bounds: bounds,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    bounds: [Vector3f; 2],
}

impl BoundingBox {
    pub fn new(min: Vector3f, max: Vector3f) -> BoundingBox {
        BoundingBox { bounds: [min, max] }
    }

    pub fn intersect(&self, ray: Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].0 - ray.origin.0) * ray.inverse_direction.0;
        let mut tmax = (self.bounds[1 - ray.sign[0]].0 - ray.origin.0) * ray.inverse_direction.0;
        let tymin = (self.bounds[ray.sign[1]].1 - ray.origin.1) * ray.inverse_direction.1;
        let tymax = (self.bounds[1 - ray.sign[1]].1 - ray.origin.1) * ray.inverse_direction.1;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].2 - ray.origin.2) * ray.inverse_direction.2;
        let tzmax = (self.bounds[1 - ray.sign[2]].2 - ray.origin.2) * ray.inverse_direction.2;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }
        
        // if tzmin > tmin {
        //     tmin = tzmin;
        // }
        // if tzmax < tmax {
        //     tmax = tzmax;
        // }

        return true;
    }
}
