use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector2f(pub f64, pub f64);

impl Vector2f {
    pub fn zero() -> Vector2f {
        Vector2f(0.0, 0.0)
    }
}
