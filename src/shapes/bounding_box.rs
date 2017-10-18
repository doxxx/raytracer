use rand;

use matrix::Matrix44f;
use point::Point;
use system::Ray;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    bounds: [Point; 2],
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox { bounds: [min, max] }
    }

    pub fn zero() -> BoundingBox {
        BoundingBox::new(Point::zero(), Point::zero())
    }

    pub fn min(&self) -> Point {
        self.bounds[0]
    }

    pub fn max(&self) -> Point {
        self.bounds[1]
    }

    pub fn x_range(&self) -> (f64,f64) {
        (self.min().x, self.max().x)
    }

    pub fn y_range(&self) -> (f64,f64) {
        (self.min().y, self.max().y)
    }

    pub fn z_range(&self) -> (f64,f64) {
        (self.min().z, self.max().z)
    }

    pub fn random_point(&self) -> Point {
        let xr = self.max().x - self.min().x;
        let yr = self.max().y - self.min().y;
        let zr = self.max().z - self.min().z;
        let x = rand::random::<f64>() * xr + self.min().x;
        let y = rand::random::<f64>() * yr + self.min().y;
        let z = rand::random::<f64>() * zr + self.min().z;
        Point::new(x, y, z)
    }

    pub fn transform(self, m: Matrix44f) -> BoundingBox {
        let a_min = self.min().as_array();
        let a_max = self.max().as_array();

        let mut b_min = (Point::zero() + m.translation_direction()).as_array();
        let mut b_max = b_min.clone();

        for i in 0..3 {
            for j in 0..3 {
                let a = m[i][j] * a_min[j];
                let b = m[i][j] * a_max[j];
                if a < b {
                    b_min[i] += a;
                    b_max[i] += b;
                } else {
                    b_min[i] += b;
                    b_max[i] += a;
                }
            }
        }

        BoundingBox::new(Point::from_array(b_min), Point::from_array(b_max))
    }

    pub fn extend(self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::new(self.min().min(other.min()), self.max().max(other.max()))
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
