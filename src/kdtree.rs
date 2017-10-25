use std::cmp::Ordering;
use std::fmt::Debug;
use std::f64;

use point::Point;

macro_rules! debug_print {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { eprintln!($($arg)*); })
}

#[derive(Clone)]
pub struct Data<T: Clone> {
    pub point: Point,
    pub value: T,
}

impl<T: Clone> Data<T> {
    pub fn new(point: Point, value: T) -> Data<T> {
        Data {
            point,
            value,
        }
    }
}

pub struct Tree<T: Clone> {
    pub data: Data<T>,
    pub left: Option<Box<Tree<T>>>,
    pub right: Option<Box<Tree<T>>>,
}

struct SearchElement<T: Clone>(f64, Data<T>);

impl<T: Clone> Tree<T> {
    pub fn new(data: &[Data<T>]) -> Option<Box<Tree<T>>> {
        let mut data = data.to_vec();
        Tree::split(&mut data, 0)
    }

    fn split(data: &mut [Data<T>], depth: usize) -> Option<Box<Tree<T>>> {
        if data.len() == 0 {
            return None;
        }

        let axis = depth % 3;
        data.sort_unstable_by(|a, b| match axis {
            0 => a.point.x.partial_cmp(&b.point.x),
            1 => a.point.y.partial_cmp(&b.point.y),
            2 => a.point.z.partial_cmp(&b.point.z),
            _ => panic!()
        }.unwrap());

        let median_index = data.len() / 2;
        let (left, right) = data.split_at_mut(median_index);
        let (median, right) = right.split_first_mut().unwrap();
        Some(Box::new(Tree {
            data: median.clone(),
            left: Tree::split(left, depth + 1),
            right: Tree::split(right, depth + 1),
        }))
    }

    pub fn find_nearest(&self, point: Point) -> Data<T> {
        self.find_nearest_n(point, 1).pop().unwrap()
    }

    pub fn find_nearest_n(&self, origin: Point, n: usize) -> Vec<Data<T>> {
        let mut r: Vec<SearchElement<T>> = Vec::with_capacity(n+1);
        self.find_nearest_n_(origin, n, &mut r, 0);
        r.into_iter().map(|n| Data::new(n.1.point, n.1.value)).collect()
    }

    fn find_nearest_n_(&self, origin: Point, n: usize, r: &mut Vec<SearchElement<T>>, depth: usize) {
        let axis = depth % 3;
        let left = match axis {
            0 => origin.x < self.data.point.x,
            1 => origin.y < self.data.point.y,
            2 => origin.z < self.data.point.z,
            _ => panic!()
        };

        let (first, second) = if left {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        let max_distance = r.last().map(|e| e.0).unwrap_or(f64::MAX);

        if let &Some(ref branch) = first {
            let d = (origin - branch.data.point).length_squared();
            if r.len() < n || d < max_distance {
                branch.find_nearest_n_(origin, n, r, depth + 1);
            }
        }

        if let &Some(ref branch) = second {
            let d = (origin - branch.data.point).length_squared();
            if r.len() < n || d < max_distance {
                branch.find_nearest_n_(origin, n, r, depth + 1);
            }
        }

        let current_distance = (origin - self.data.point).length_squared();
        let element = SearchElement(current_distance, self.data.clone());
        let max_distance = Tree::add_if_nearest(origin, n, r, element);
    }

    /// insertion sort, returns max distance in vec after insertion
    fn add_if_nearest(origin: Point, n: usize, r: &mut Vec<SearchElement<T>>, element: SearchElement<T>) {
        let mut index = None;
        for i in 0..r.len() {
            if let Some(order) = (&element.0).partial_cmp(&r[i].0) {
                if order == Ordering::Less {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            r.insert(i, element);
            r.truncate(n);
        } else if r.len() < n {
            r.push(element);
        }
    }

    pub fn find_within_radius(&self, point: Point, radius: f64) -> Vec<Data<T>> {
        let mut r: Vec<Data<T>> = Vec::new();
        self.find_within_radius_(point, radius.powi(2), &mut r, 0);
        r
    }

    fn find_within_radius_(&self, point: Point, radius_sq: f64, r: &mut Vec<Data<T>>, depth: usize) {
        let axis = depth % 3;

        let distance_sq = (point - self.data.point).length_squared();
        if distance_sq <= radius_sq {
            r.push(self.data.clone());
        }

        let left = match axis {
            0 => point.x < self.data.point.x,
            1 => point.y < self.data.point.y,
            2 => point.z < self.data.point.z,
            _ => panic!()
        };

        let branch = if left { &self.left } else { &self.right };
        if let &Some(ref branch) = branch {
            branch.find_within_radius_(point, radius_sq, r, depth + 1);
        }

        let other_branch = if left { &self.right } else { &self.left };
        if let &Some(ref other_branch) = other_branch {
            let other_distance = (point - other_branch.data.point).length_squared();
            if other_distance < radius_sq {
                other_branch.find_within_radius_(point, radius_sq, r, depth + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_if_nearest() {
        let mut r = vec![
            SearchElement(1.0, Data::new(Point::new(1.0, 0.0, 0.0), 1)),
            SearchElement(2.0, Data::new(Point::new(2.0, 0.0, 0.0), 2)),
            SearchElement(3.0, Data::new(Point::new(3.0, 0.0, 0.0), 3)),
        ];

        Tree::add_if_nearest(Point::zero(), 4, &mut r, SearchElement(4.0, Data::new(Point::new(4.0, 0.0, 0.0), 4)));
        let values: Vec<_> = r.iter().map(|e| e.1.value).collect();
        assert_eq!(values, vec![1, 2, 3, 4]);

        Tree::add_if_nearest(Point::zero(), 4, &mut r, SearchElement(1.5, Data::new(Point::new(1.5, 0.0, 0.0), 5)));
        let values: Vec<_> = r.iter().map(|e| e.1.value).collect();
        assert_eq!(values, vec![1, 5, 2, 3]);

        Tree::add_if_nearest(Point::zero(), 4, &mut r, SearchElement(0.5, Data::new(Point::new(0.5, 0.0, 0.0), 6)));
        let values: Vec<_> = r.iter().map(|e| e.1.value).collect();
        assert_eq!(values, vec![6, 1, 5, 2]);
    }

    #[test]
    fn construction() {
        let data = vec![
            Data::new(Point::new(2.0, 3.0, 1.0), "dog"),
            Data::new(Point::new(5.0, 2.0, 6.0), "cat"),
            Data::new(Point::new(1.0, 9.0, 2.0), "rat"),
            Data::new(Point::new(4.0, 1.0, 5.0), "mog"),
        ];

        let tree = Tree::new(&data);
        assert!(tree.is_some());
        let tree = tree.unwrap();

        assert_eq!(tree.data.point, data[3].point);
        assert_eq!(tree.data.value, data[3].value);

        {
            let tree = &tree.left;
            assert!(tree.is_some());
            let tree = tree.as_ref().unwrap();

            assert_eq!(tree.data.point, data[2].point);
            assert_eq!(tree.data.value, data[2].value);

            {
                let tree = &tree.left;
                assert!(tree.is_some());
                let tree = tree.as_ref().unwrap();

                assert_eq!(tree.data.point, data[0].point);
                assert_eq!(tree.data.value, data[0].value);
            }
        }

        {
            let tree = &tree.right;
            assert!(tree.is_some());
            let tree = tree.as_ref().unwrap();

            assert_eq!(tree.data.point, data[1].point);
            assert_eq!(tree.data.value, data[1].value);
        }
    }

    #[test]
    fn find_nearest() {
        let data = vec![
            Data::new(Point::new(2.0, 3.0, 1.0), "dog"),
            Data::new(Point::new(5.0, 2.0, 6.0), "cat"),
            Data::new(Point::new(1.0, 9.0, 2.0), "rat"),
            Data::new(Point::new(4.0, 1.0, 5.0), "mog"),
        ];

        let tree = Tree::new(&data).unwrap();

        let nearest = tree.find_nearest(Point::new(2.0, 3.0, 1.0));
        assert_eq!(nearest.value, "dog");

        let nearest = tree.find_nearest(Point::new(5.0, 2.0, 6.0));
        assert_eq!(nearest.value, "cat");

        let nearest = tree.find_nearest(Point::new(1.0, 9.0, 2.0));
        assert_eq!(nearest.value, "rat");

        let nearest = tree.find_nearest(Point::new(4.0, 1.0, 5.0));
        assert_eq!(nearest.value, "mog");

        let nearest = tree.find_nearest(Point::new(0.0, 0.0, 0.0));
        assert_eq!(nearest.value, "dog");

        let nearest = tree.find_nearest(Point::new(2.0, 4.0, 1.0));
        assert_eq!(nearest.value, "dog");
    }

    #[test]
    fn find_nearest_n() {
        let data = vec![
            Data::new(Point::new(2.0, 3.0, 1.0), "dog"),
            Data::new(Point::new(5.0, 2.0, 6.0), "cat"),
            Data::new(Point::new(1.0, 9.0, 2.0), "rat"),
            Data::new(Point::new(4.0, 1.0, 5.0), "mog"),
        ];

        let tree = Tree::new(&data).unwrap();

        let nearest = tree.find_nearest_n(Point::new(2.0, 3.0, 1.0), 1).into_iter().map(|a| a.value).collect::<Vec<_>>();
        assert_eq!(nearest, vec!["dog"]);

        let nearest = tree.find_nearest_n(Point::new(2.0, 3.0, 1.0), 2).into_iter().map(|a| a.value).collect::<Vec<_>>();
        assert_eq!(nearest, vec!["dog", "mog"]);

        let nearest = tree.find_nearest_n(Point::new(2.0, 3.0, 1.0), 3).into_iter().map(|a| a.value).collect::<Vec<_>>();
        assert_eq!(nearest, vec!["dog", "mog", "cat"]);

        let nearest = tree.find_nearest_n(Point::new(2.0, 3.0, 1.0), 4).into_iter().map(|a| a.value).collect::<Vec<_>>();
        assert_eq!(nearest, vec!["dog", "mog", "cat", "rat"]);
    }

    #[test]
    fn find_within_radius() {
        let data = vec![
            Data::new(Point::new(0.0, 0.0, 0.0), "close"),
            Data::new(Point::new(0.1, 0.1, 0.1), "close"),
            Data::new(Point::new(-0.1, -0.1, -0.1), "close"),
            Data::new(Point::new(5.0, 5.0, 5.0), "far"),
            Data::new(Point::new(-5.0, -5.0, -5.0), "far"),
        ];

        let tree = Tree::new(&data).unwrap();

        let nearest = tree.find_within_radius(Point::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(nearest.len(), 3);
        assert!(nearest.iter().all(|d| d.value == "close"));

        let nearest = tree.find_within_radius(Point::new(0.0, 0.0, 0.0), 10.0);
        assert_eq!(nearest.len(), 5);
        assert!(nearest.iter().all(|d| d.value == "close" || d.value == "far"));
    }
}

#[cfg(all(feature = "benchmarks", test))]
mod benchmarks {
    use super::*;

    use std::time::{Duration, Instant};
    use rand::random;

    #[test]
    fn construction() {
        let mut data: Vec<Data<usize>> = Vec::new();

        for i in 0..100000 {
            data.insert(i, Data::new(random_point_in_cube(100.0), i));
        }

        let elapsed = bench(100, || Tree::new(&data));

        eprintln!("construction: avg={}", pretty_print_ns(elapsed))
    }

    #[test]
    fn find_nearest() {
        let mut data: Vec<Data<usize>> = Vec::new();

        for i in 0..100000 {
            data.insert(i, Data::new(random_point_in_cube(100.0), i));
        }

        let tree = Tree::new(&data).unwrap();

        let elapsed = bench(100000, || tree.find_nearest(random_point_in_cube(100.0)));

        eprintln!("find_nearest: avg={}", pretty_print_ns(elapsed))
    }

    #[test]
    fn find_within_radius() {
        let mut data: Vec<Data<usize>> = Vec::new();

        for i in 0..100000 {
            data.insert(i, Data::new(random_point_in_cube(100.0), i));
        }

        let tree = Tree::new(&data).unwrap();

        let elapsed = bench(10000, || tree.find_within_radius(random_point_in_cube(100.0), 50.0));

        eprintln!("find_within_radius: avg={}", pretty_print_ns(elapsed))
    }

    fn random_point_in_cube(side_length: f64) -> Point {
        Point::new(
            random::<f64>() * side_length - side_length / 2.0,
            random::<f64>() * side_length - side_length / 2.0,
            random::<f64>() * side_length - side_length / 2.0
        )
    }

    fn bench<F, T>(count: u64, f: F) -> u64 where F: Fn() -> T {
        let start = Instant::now();

        for _ in 0..count {
            black_box(f());
        }

        ns_from_dur(start.elapsed()) / count
    }

    #[inline(never)]
    pub fn black_box<T>(dummy: T) -> T {
        dummy
    }

    fn ns_from_dur(dur: Duration) -> u64 {
        dur.as_secs() * 1_000_000_000 + (dur.subsec_nanos() as u64)
    }

    fn pretty_print_ns(ns: u64) -> String {
        let ms = ns / 1_000_000;
        if ms > 0 {
            return format!("{}ms", ms)
        }

        let us = ns / 1_000;
        if us > 0 {
            return format!("{}μs", us)
        }

        format!("{}ns", ns)
    }
}
