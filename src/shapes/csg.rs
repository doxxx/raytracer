use point::Point;
use shapes::bounding_box::BoundingBox;
use shapes::{Interval, Shape};
use system::{Intersectable, Intersection, Ray};

/// Constructive Solid Geometry Union
pub struct CSGUnion {
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGUnion {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGUnion {
        CSGUnion {
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
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGIntersection {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGIntersection {
        CSGIntersection {
            a,
            b,
        }
    }
}

impl Shape for CSGIntersection {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let intervals_a = self.a.intersection_intervals(ray);
        let intervals_b = self.b.intersection_intervals(ray);

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

        intervals.sort_by(|a,b| a.partial_cmp(b).unwrap());

        intervals
    }
}

impl Intersectable for CSGIntersection {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}

/// Constructive Solid Geometry Difference
pub struct CSGDifference {
    a: Box<Shape>,
    b: Box<Shape>,
}

impl CSGDifference {
    pub fn new(a: Box<Shape>, b: Box<Shape>) -> CSGDifference {
        CSGDifference {
            a,
            b,
        }
    }
}

impl Shape for CSGDifference {
    fn intersection_intervals(&self, ray: &Ray) -> Vec<Interval> {
        let intervals_a = self.a.intersection_intervals(ray);
        let intervals_b = self.b.intersection_intervals(ray);

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
                new_start.n *=-1.0;
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

        intervals.sort_by(|a,b| a.partial_cmp(b).unwrap());

        intervals
    }
}

impl Intersectable for CSGDifference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // if !self.bounds.intersect(ray) {
        //     return None;
        // }

        super::first_positive_intersection(self.intersection_intervals(ray))
    }
}
