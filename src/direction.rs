use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Direction {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Direction {
        Direction {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Direction {
        Direction::new(0.0, 0.0, 0.0)
    }

    pub fn cross(&self, rhs: Direction) -> Direction {
        Direction::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn normalize(self) -> Direction {
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
            if self.x < 0.0 { 1 } else { 0 },
            if self.y < 0.0 { 1 } else { 0 },
            if self.z < 0.0 { 1 } else { 0 },
        ]
    }

    pub fn reflect(self, normal: Direction) -> Direction {
        self - normal * 2.0 * self.dot(normal)
    }
}

pub trait Dot<RHS=Self> {
    fn dot(&self, rhs: RHS) -> f64;
}

impl Dot for Direction {
    fn dot(&self, rhs: Direction) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Dot<Point> for Direction {
    fn dot(&self, rhs: Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        Direction::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, rhs: Direction) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Direction {
    type Output = Direction;

    fn sub(self, rhs: Direction) -> Self::Output {
        Direction::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Direction {
    fn sub_assign(&mut self, rhs: Direction) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Direction {
    type Output = Direction;

    fn mul(self, rhs: Direction) -> Self::Output {
        Direction::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Direction {
    type Output = Direction;

    fn mul(self, rhs: f64) -> Self::Output {
        Direction::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Direction> for f64 {
    type Output = Direction;

    fn mul(self, rhs: Direction) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Direction {
    fn mul_assign(&mut self, rhs: Direction) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Direction {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Direction {
    type Output = Direction;

    fn div(self, rhs: Direction) -> Self::Output {
        Direction::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Direction {
    type Output = Direction;

    fn div(self, rhs: f64) -> Self::Output {
        Direction::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Direction> for f64 {
    type Output = Direction;

    fn div(self, rhs: Direction) -> Self::Output {
        Direction::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl DivAssign for Direction {
    fn div_assign(&mut self, rhs: Direction) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f64> for Direction {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Direction {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}
