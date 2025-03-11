use crate::matrix::Matrix44f;
use crate::object::Transformation;
use crate::shapes::{first_positive_intersection, Interval, Shape};
use crate::system::{Intersectable, Intersection, Ray, Transformable};

/// Constructive Solid Geometry Union
pub struct CSGUnion {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
    tx: Transformation,
}

impl CSGUnion {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> CSGUnion {
        CSGUnion { a, b, tx: Transformation::new() }
    }
}

impl Shape for CSGUnion {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let object_ray = ray.to_object(&self.tx);
        let intervals_a = self.a.intersection_intervals(&object_ray);
        let intervals_b = self.b.intersection_intervals(&object_ray);

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

        if let Some(i) = interval_a {
            intervals.push(i);
        } else if let Some(i) = interval_b {
            intervals.push(i);
        }

        while let Some(i) = iter_a.next() {
            intervals.push(i);
        }

        while let Some(i) = iter_b.next() {
            intervals.push(i);
        }

        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intervals.into_iter().map(|i| i.to_world(ray, &object_ray, &self.tx)).collect()
    }
}

impl Intersectable for CSGUnion {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        first_positive_intersection(self.intersection_intervals(ray))
    }
}

/// Constructive Solid Geometry Intersection
pub struct CSGIntersection {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
    tx: Transformation,
}

impl CSGIntersection {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> CSGIntersection {
        CSGIntersection { a, b, tx: Transformation::new() }
    }
}

impl Shape for CSGIntersection {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let object_ray = ray.to_object(&self.tx);
        let intervals_a = self.a.intersection_intervals(&object_ray);
        let intervals_b = self.b.intersection_intervals(&object_ray);

        if intervals_a.len() == 0 || intervals_b.len() == 0 {
            return Vec::with_capacity(0);
        }

        let mut intervals = Vec::new();
        let mut iter_a = intervals_a.into_iter();
        let mut iter_b = intervals_b.into_iter();
        let mut interval_a = iter_a.next();
        let mut interval_b = iter_b.next();

        while let (Some(Interval(a_start, a_end)), Some(Interval(b_start, b_end))) = (interval_a, interval_b) {
            if a_end < b_start {
                // interval_a ends before interval_b starts
                interval_a = iter_a.next();
            } else if b_end < a_start {
                // interval_b ends before interval_a starts
                interval_b = iter_b.next();
            } else {
                // intervals intersect
                let new_interval = Interval(
                    if a_start > b_start { a_start } else { b_start },
                    if a_end < b_end { a_end } else { b_end },
                );
                intervals.push(new_interval);
                if b_end >= a_end {
                    interval_a = iter_a.next();
                } else {
                    interval_b = iter_b.next();
                }
            }
        }

        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intervals.into_iter().map(|i| i.to_world(ray, &object_ray, &self.tx)).collect()
    }
}

impl Intersectable for CSGIntersection {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        first_positive_intersection(self.intersection_intervals(ray))
    }
}

/// Constructive Solid Geometry Difference
pub struct CSGDifference {
    a: Box<dyn Shape>,
    b: Box<dyn Shape>,
    tx: Transformation,
}

impl CSGDifference {
    pub fn new(a: Box<dyn Shape>, b: Box<dyn Shape>) -> CSGDifference {
        CSGDifference { a, b, tx: Transformation::new() }
    }
}

impl Shape for CSGDifference {
    fn transform(&mut self, m: Matrix44f) {
        self.tx.transform(m);
    }

    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let object_ray = ray.to_object(&self.tx);
        let intervals_a = self.a.intersection_intervals(&object_ray);
        let intervals_b = self.b.intersection_intervals(&object_ray);

        if intervals_a.len() == 0 {
            return Vec::with_capacity(0);
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
                interval_b = iter_b.next();
            } else if b_start <= a_start && b_end >= a_end {
                // interval_b covers or exceeds interval_a
                interval_a = iter_a.next();
            } else if b_start <= a_start && b_end < a_end {
                // interval_b overlaps interval_a start
                let mut new_start = b_end;
                new_start.n *= -1.0;
                interval_a = Some(Interval(new_start, a_end));
                interval_b = iter_b.next();
            } else if b_start < a_end && b_end < a_end {
                // interval_b overlaps middle of interval_a
                let mut a_1_end = b_start;
                a_1_end.n *= -1.0;
                intervals.push(Interval(a_start, a_1_end));
                let mut a_2_start = b_end;
                a_2_start.n *= -1.0;
                interval_a = Some(Interval(a_2_start, a_end));
                interval_b = iter_b.next();
            } else if b_start < a_end && b_end >= a_end {
                // interval_b overlaps interval_a end
                let mut new_end = b_start;
                new_end.n *= -1.0;
                intervals.push(Interval(a_start, new_end));
                interval_a = iter_a.next();
            } else {
                panic!("unanticipated difference case");
            }
        }

        if let Some(i) = interval_a {
            intervals.push(i);
        }

        while let Some(i) = iter_a.next() {
            intervals.push(i);
        }

        intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());

        intervals.into_iter().map(|i| i.to_world(ray, &object_ray, &self.tx)).collect()
    }
}

impl Intersectable for CSGDifference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        first_positive_intersection(self.intersection_intervals(ray))
    }
}
