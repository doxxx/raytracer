use std::cmp::PartialEq;
use std::f64;
use std::ops::{Index, IndexMut, Mul};

use direction::Direction;
use point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Matrix44f(pub [[f64; 4]; 4]);

impl Matrix44f {
    pub fn zero() -> Matrix44f {
        Matrix44f([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn identity() -> Matrix44f {
        Matrix44f([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(d: Direction) -> Matrix44f {
        Matrix44f([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [d.x, d.y, d.z, 1.0],
        ])
    }

    pub fn scaling(d: Direction) -> Matrix44f {
        Matrix44f([
            [d.x, 0.0, 0.0, 0.0],
            [0.0, d.y, 0.0, 0.0],
            [0.0, 0.0, d.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(deg: f64) -> Matrix44f {
        let (sin, cos) = deg.to_radians().sin_cos();
        Matrix44f([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, sin, 0.0],
            [0.0, -sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(deg: f64) -> Matrix44f {
        let (sin, cos) = deg.to_radians().sin_cos();
        Matrix44f([
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(deg: f64) -> Matrix44f {
        let (sin, cos) = deg.to_radians().sin_cos();
        Matrix44f([
            [cos, sin, 0.0, 0.0],
            [-sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn row(&self, i: usize) -> [f64; 4] {
        self.0[i]
    }

    pub fn inverse(&self) -> Matrix44f {
        let mut s = Matrix44f::identity();
        let mut t = self.clone();

        // forward elimination
        for i in 0..3 {
            let mut pivot = i;
            let mut pivot_size = t[i][i];

            if pivot_size < 0.0 {
                pivot_size = -pivot_size;
            }

            for j in (i + 1)..4 {
                let mut tmp = t[j][i];
                if tmp < 0.0 {
                    tmp = -tmp;
                }
                if tmp > pivot_size {
                    pivot = j;
                    pivot_size = tmp;
                }
            }

            if pivot_size == 0.0 {
                // cannot invert singular matrix
                return Matrix44f::identity();
            }

            if pivot != i {
                for j in 0..4 {
                    let mut tmp = t[i][j];
                    t[i][j] = t[pivot][j];
                    t[pivot][j] = tmp;

                    tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;
                }
            }

            for j in (i + 1)..4 {
                let f = t[j][i] / t[i][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        // backward substitution
        for i in (0..4).rev() {
            let mut f = t[i][i];
            if f == 0.0 {
                // cannot invert singular matrix
                return Matrix44f::identity();
            }

            for j in 0..4 {
                t[i][j] /= f;
                s[i][j] /= f;
            }

            for j in 0..i {
                f = t[j][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        s

    }

    pub fn transpose(&self) -> Matrix44f {
        let mut t = Matrix44f::zero();
        for i in 0..4 {
            for j in 0..4 {
                t[i][j] = self[j][i];
            }
        }
        t
    }
}

impl Index<usize> for Matrix44f {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix44f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul for Matrix44f {
    type Output = Self;

    fn mul(self, rhs: Matrix44f) -> Self::Output {
        let mut result = Matrix44f::zero();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * rhs[0][j] +
                    self[i][1] * rhs[1][j] +
                    self[i][2] * rhs[2][j] +
                    self[i][3] * rhs[3][j]
            }
        }
        result
    }
}

impl Mul<Matrix44f> for Point {
    type Output = Point;

    fn mul(self, rhs: Matrix44f) -> Self::Output {
        let mut v = Point::new(
            self.x * rhs[0][0] + self.y * rhs[1][0] + self.z * rhs[2][0] + rhs[3][0],
            self.x * rhs[0][1] + self.y * rhs[1][1] + self.z * rhs[2][1] + rhs[3][1],
            self.x * rhs[0][2] + self.y * rhs[1][2] + self.z * rhs[2][2] + rhs[3][2],
        );
        let w = self.x * rhs[0][3] + self.y * rhs[1][3] + self.z * rhs[2][3] + rhs[3][3];
        if w != 1.0 && w != 0.0 {
            v /= w
        }
        v
    }
}

impl Mul<Matrix44f> for Direction {
    type Output = Direction;
    
    fn mul(self, rhs: Matrix44f) -> Self::Output {
        Direction::new(
            self.x * rhs[0][0] + self.y * rhs[1][0] + self.z * rhs[2][0],
            self.x * rhs[0][1] + self.y * rhs[1][1] + self.z * rhs[2][1],
            self.x * rhs[0][2] + self.y * rhs[1][2] + self.z * rhs[2][2],
        )
    }
}

const EPSILON: f64 = (f64::EPSILON * 100.0);

impl PartialEq for Matrix44f {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self[i][j] - other[i][j] > EPSILON {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix44f;
    use direction::Direction;
    use point::Point;
    use test_utils::ApproxEq;

    #[test]
    fn inverse_identity() {
        let m = Matrix44f::identity();
        let inv = m.inverse();
        assert_eq!(m, inv);
    }

    #[test]
    fn inverse_non_identity() {
        let m = Matrix44f(
            [
                [1.0, 3.0, 2.0, 4.0],
                [4.0, 2.0, 3.0, 5.0],
                [5.0, 4.0, 3.0, 1.0],
                [3.0, 1.0, 2.0, 4.0],
            ],
        );
        let expected = Matrix44f(
            [
                [0.0, -20.0 / 12.0, 4.0 / 12.0, 24.0 / 12.0],
                [6.0 / 12.0, -20.0 / 12.0, 4.0 / 12.0, 18.0 / 12.0],
                [-9.0 / 12.0, 64.0 / 12.0, -8.0 / 12.0, -69.0 / 12.0],
                [3.0 / 12.0, -12.0 / 12.0, 0.0, 15.0 / 12.0],
            ],
        );
        let actual = m.inverse();
        assert_approx_eq!(actual, expected);
        let identity = m * actual;
        assert_approx_eq!(identity, Matrix44f::identity());
    }

    #[test]
    pub fn point_translation() {
        let p = Point::new(0.1, 2.3, 4.5);
        let m = Matrix44f::translation(Direction::new(1.2, 3.4, 5.6));
        let actual = p * m;
        let expected = Point::new(1.3, 5.7, 10.1);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_rotation_x() {
        let p = Point::new(0.0, 1.0, 0.0);
        let m = Matrix44f::rotation_x(90.0);
        let actual = p * m;
        let expected = Point::new(0.0, 0.0, 1.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_rotation_y() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_y(90.0);
        let actual = p * m;
        let expected = Point::new(0.0, 0.0, -1.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_rotation_z() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_z(90.0);
        let actual = p * m;
        let expected = Point::new(0.0, 1.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_scaling() {
        let p = Point::new(1.0, 1.0, 1.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 3.0, 4.0));
        let actual = p * m;
        let expected = Point::new(2.0, 3.0, 4.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_translation_then_rotation() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::translation(Direction::new(2.0, 0.0, 0.0)) * Matrix44f::rotation_z(90.0);
        let actual = p * m;
        let expected = Point::new(0.0, 3.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_rotation_then_translation() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_z(90.0) * Matrix44f::translation(Direction::new(2.0, 0.0, 0.0));
        let actual = p * m;
        let expected = Point::new(2.0, 1.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_scaling_then_rotation() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0)) * Matrix44f::rotation_y(90.0);
        let actual = p * m;
        let expected = Point::new(0.0, 0.0, -2.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_rotation_then_scaling() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_y(90.0) * Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0));
        let actual = p * m;
        let expected = Point::new(0.0, 0.0, -2.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_scaling_then_translation() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0)) * Matrix44f::translation(Direction::new(0.0, 4.0, 3.0));
        let actual = p * m;
        let expected = Point::new(2.0, 4.0, 3.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn point_translation_then_scaling() {
        let p = Point::new(1.0, 0.0, 0.0);
        let m = Matrix44f::translation(Direction::new(0.0, 4.0, 3.0)) * Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0));
        let actual = p * m;
        let expected = Point::new(2.0, 8.0, 6.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_translation() {
        let d = Direction::new(0.1, 2.3, 4.5);
        let m = Matrix44f::translation(Direction::new(1.2, 3.4, 5.6));
        let actual = d * m;
        let expected = d;
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_rotation_x() {
        let d = Direction::new(0.0, 1.0, 0.0);
        let m = Matrix44f::rotation_x(90.0);
        let actual = d * m;
        let expected = Direction::new(0.0, 0.0, 1.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_rotation_y() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_y(90.0);
        let actual = d * m;
        let expected = Direction::new(0.0, 0.0, -1.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_rotation_z() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_z(90.0);
        let actual = d * m;
        let expected = Direction::new(0.0, 1.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_scaling() {
        let d = Direction::new(1.0, 1.0, 1.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 3.0, 4.0));
        let actual = d * m;
        let expected = Direction::new(2.0, 3.0, 4.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_translation_then_rotation() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::translation(Direction::new(2.0, 0.0, 0.0)) * Matrix44f::rotation_z(90.0);
        let actual = d * m;
        let expected = Direction::new(0.0, 1.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_rotation_then_translation() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_z(90.0) * Matrix44f::translation(Direction::new(2.0, 0.0, 0.0));
        let actual = d * m;
        let expected = Direction::new(0.0, 1.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_scaling_then_rotation() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0)) * Matrix44f::rotation_y(90.0);
        let actual = d * m;
        let expected = Direction::new(0.0, 0.0, -2.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_rotation_then_scaling() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::rotation_y(90.0) * Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0));
        let actual = d * m;
        let expected = Direction::new(0.0, 0.0, -2.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_scaling_then_translation() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0)) * Matrix44f::translation(Direction::new(0.0, 4.0, 3.0));
        let actual = d * m;
        let expected = Direction::new(2.0, 0.0, 0.0);
        assert_approx_eq!(actual, expected);
    }

    #[test]
    pub fn dir_translation_then_scaling() {
        let d = Direction::new(1.0, 0.0, 0.0);
        let m = Matrix44f::translation(Direction::new(0.0, 4.0, 3.0)) * Matrix44f::scaling(Direction::new(2.0, 2.0, 2.0));
        let actual = d * m;
        let expected = Direction::new(2.0, 0.0, 0.0);
        assert_approx_eq!(actual, expected);
    }
}
