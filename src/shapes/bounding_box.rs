use crate::point::Point;
use crate::system::Ray;

pub struct BoundingBox {
    bounds: [Point; 2],
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox { bounds: [min, max] }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut tmin = (self.bounds[ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let mut tmax = (self.bounds[1 - ray.sign[0]].x - ray.origin.x) * ray.inverse_direction.x;
        let tymin = (self.bounds[ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;
        let tymax = (self.bounds[1 - ray.sign[1]].y - ray.origin.y) * ray.inverse_direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds[ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;
        let tzmax = (self.bounds[1 - ray.sign[2]].z - ray.origin.z) * ray.inverse_direction.z;

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
