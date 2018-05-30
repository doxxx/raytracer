use point::Point;
use shapes::bounding_box::BoundingBox;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray};

/// Constructive Solid Geometry Union
pub struct CSGUnion {
    bounds: BoundingBox,
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGUnion {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGUnion {
        CSGUnion {
            bounds: BoundingBox::new(Point::zero(), Point::zero()),
            a,
            b,
        }
    }
}

impl Shape for CSGUnion {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let intervals_a = self.a.intersection_intervals(ray);
        let intervals_b = self.b.intersection_intervals(ray);

        if intervals_a.len() == 0 {
            return intervals_b;
        }
        if intervals_b.len() == 0 {
            return intervals_a;
        }

        let mut intervals = Vec::new();
        let mut iter_a = intervals_a.into_iter();
        let mut iter_b = intervals_b.into_iter();
        let mut interval_a = iter_a.next();
        let mut interval_b = iter_b.next();

        while let (Some(Interval(a_start, a_end)), Some(Interval(b_start, b_end))) = (interval_a, interval_b) {
            if a_end < b_start {
                // interval_a ends before interval_b starts
                intervals.push(interval_a.unwrap());
                interval_a = iter_a.next();
            } else if b_end < a_start {
                // interval_b ends before interval_a starts
                intervals.push(interval_b.unwrap());
                interval_b = iter_b.next();
            } else {
                // intervals intersect
                let mut new_start = a_start;
                let mut new_end = a_end;
                if b_start < new_start {
                    new_start = b_start;
                }
                if b_end > new_end {
                    new_end = b_end;
                    interval_a = iter_a.next();
                    while let Some(Interval(a_start, a_end)) = interval_a {
                        if a_start > new_end {
                            break;
                        }
                        new_end = a_end;
                        interval_a = iter_a.next();
                    }
                }
                intervals.push(Interval(new_start, new_end));
                interval_b = iter_b.next();
            }
        }

        while let Some(interval_a) = iter_a.next() {
            intervals.push(interval_a.clone());
        }

        while let Some(interval_b) = iter_b.next() {
            intervals.push(interval_b.clone());
        }

        intervals.sort_by(|a,b| a.partial_cmp(b).unwrap());

        intervals
    }
}

impl Intersectable for CSGUnion {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

/// Constructive Solid Geometry Intersection
pub struct CSGIntersection {
    bounds: BoundingBox,
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGIntersection {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGUnion {
        CSGUnion {
            bounds: BoundingBox::new(Point::zero(), Point::zero()),
            a,
            b,
        }
    }
}

impl Shape for CSGIntersection {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        Vec::with_capacity(0)
    }
}

impl Intersectable for CSGIntersection {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounds.intersect(ray) {
            return None;
        }

        None
    }
}

/// Constructive Solid Geometry Difference
pub struct CSGDifference {
    bounds: BoundingBox,
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGDifference {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGUnion {
        CSGUnion {
            bounds: BoundingBox::new(Point::zero(), Point::zero()),
            a,
            b,
        }
    }
}

impl Shape for CSGDifference {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        Vec::with_capacity(0)
    }
}

impl Intersectable for CSGDifference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if !self.bounds.intersect(ray) {
            return None;
        }

        None
    }
}
