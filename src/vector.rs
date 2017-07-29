use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector2f(pub f64, pub f64);

impl Vector2f {
    pub fn zero() -> Vector2f {
        Vector2f(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3f(pub f64, pub f64, pub f64);

impl Vector3f {
    pub fn zero() -> Vector3f {
        Vector3f(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, other: Vector3f) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vector3f) -> Vector3f {
        Vector3f(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn normalize(self) -> Vector3f {
        let l2 = self.length_squared();
        if l2 > 0.0 {
            let inv = 1.0 / l2.sqrt();
            self * inv
        } else {
            self
        }
    }

    pub fn sign(self) -> [usize; 3] {
        [
            if self.0 < 0.0 { 1 } else { 0 },
            if self.1 < 0.0 { 1 } else { 0 },
            if self.2 < 0.0 { 1 } else { 0 },
        ]
    }
}

impl Add for Vector3f {
    type Output = Vector3f;

    fn add(self, other: Vector3f) -> Vector3f {
        Vector3f(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vector3f {
    fn add_assign(&mut self, other: Vector3f) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(self, other: Vector3f) -> Vector3f {
        Vector3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vector3f {
    fn sub_assign(&mut self, other: Vector3f) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul for Vector3f {
    type Output = Vector3f;

    fn mul(self, other: Vector3f) -> Vector3f {
        Vector3f(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vector3f {
    type Output = Vector3f;

    fn mul(self, other: f64) -> Vector3f {
        Vector3f(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vector3f> for f64 {
    type Output = Vector3f;

    fn mul(self, other: Vector3f) -> Vector3f {
        other * self
    }
}

impl MulAssign for Vector3f {
    fn mul_assign(&mut self, other: Vector3f) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl MulAssign<f64> for Vector3f {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div for Vector3f {
    type Output = Vector3f;

    fn div(self, other: Vector3f) -> Vector3f {
        Vector3f(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f64> for Vector3f {
    type Output = Vector3f;

    fn div(self, other: f64) -> Vector3f {
        Vector3f(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Div<Vector3f> for f64 {
    type Output = Vector3f;

    fn div(self, other: Vector3f) -> Vector3f {
        Vector3f(self / other.0, self / other.1, self / other.2)
    }
}

impl DivAssign for Vector3f {
    fn div_assign(&mut self, other: Vector3f) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl DivAssign<f64> for Vector3f {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Vector3f {
        Vector3f(-self.0, -self.1, -self.2)
    }
}

impl PartialEq for Vector3f {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
