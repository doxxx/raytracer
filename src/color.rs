use std::f64;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color { Color::new(0.0, 0.0, 0.0) }
    pub fn white() -> Color { Color::new(1.0, 1.0, 1.0) }
    pub fn red() -> Color { Color::new(1.0, 0.0, 0.0) }
    pub fn green() -> Color { Color::new(0.0, 1.0, 0.0) }
    pub fn blue() -> Color { Color::new(0.0, 0.0, 1.0) }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.r - other.r) <= f64::EPSILON && (self.g - other.g) <= f64::EPSILON && (self.b - other.b) <= f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.4, 0.5, 0.6);
        let r = a + b;
        assert_eq!(Color::new(0.1 + 0.4, 0.2 + 0.5, 0.3 + 0.6), r);
    }

    #[test]
    fn add_assign() {
        let mut r = Color::new(0.1, 0.2, 0.3);
        r += Color::new(0.4, 0.5, 0.6);
        assert_eq!(Color::new(0.5, 0.7, 0.9), r);
    }

    #[test]
    fn mul_color() {
        let a = Color::new(0.1, 0.2, 0.3);
        let b = Color::new(0.4, 0.5, 0.6);
        let r = a * b;
        assert_eq!(Color::new(0.1 * 0.4, 0.2 * 0.5, 0.3 * 0.6), r);
    }

    #[test]
    fn mul_f64() {
        let c = Color::new(0.1, 0.2, 0.3);
        let r = c * 2.0;
        assert_eq!(Color::new(0.1 * 2.0, 0.2 * 2.0, 0.3 * 2.0), r);
    }

    #[test]
    fn div_f64() {
        let c = Color::new(0.1, 0.2, 0.3);
        let r = c / 2.0;
        assert_eq!(Color::new(0.1 / 2.0, 0.2 / 2.0, 0.3 / 2.0), r);
    }
}
