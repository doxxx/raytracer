use std::fmt::Debug;

use direction::Direction;

const TEST_EPSILON: f64 = 0.000001;

pub trait ApproxEq: Debug {
    fn approx_eq(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        if self.is_nan() || other.is_nan() {
            return false;
        }
        (*self - *other).abs() < TEST_EPSILON
    }
}

impl<T> ApproxEq for [T]
where
    T: ApproxEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a.approx_eq(b))
    }
}

impl<T> ApproxEq for Vec<T>
where
    T: ApproxEq,
{
    fn approx_eq(&self, other: &Self) -> bool {
        self.as_slice().approx_eq(other.as_slice())
    }
}

impl ApproxEq for Direction {
    fn approx_eq(&self, other: &Self) -> bool {
        let a = [self.x, self.y, self.z];
        let b = [other.x, other.y, other.z];

        a.approx_eq(&b)
    }
}

macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        if !$a.approx_eq($b) {
            panic!(
                "assertion failed: `(left == right)`\n   left: `{:?}`,\n  right: `{:?}`",
                $a, $b,
            )
        }
    };
}
