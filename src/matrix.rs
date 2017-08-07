use std::cmp::PartialEq;
use std::f64;
use std::ops::{Index, IndexMut, Mul};

use vector::Vector3f;

#[derive(Debug, Clone, Copy)]
pub struct Matrix44f([[f64; 4]; 4]);

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

    pub fn translation(v: Vector3f) -> Matrix44f {
        Matrix44f([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [v.0, v.1, v.2, 1.0],
        ])
    }

    pub fn scaling(v: Vector3f) -> Matrix44f {
        Matrix44f([
            [v.0, 0.0, 0.0, 0.0],
            [0.0, v.1, 0.0, 0.0],
            [0.0, 0.0, v.2, 0.0],
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

    pub fn transposed(&self) -> Matrix44f {
        let mut t = Matrix44f::zero();
        for i in 0..4 {
            for j in 0..4 {
                t[i][j] = self[j][i];
            }
        }
        t
    }

    pub fn mult_normal(self, n: Vector3f) -> Vector3f {
        Vector3f(
            n.0 * self[0][0] + n.1 * self[1][0] + n.2 * self[2][0] + self[3][0],
            n.0 * self[0][1] + n.1 * self[1][1] + n.2 * self[2][1] + self[3][1],
            n.0 * self[0][2] + n.1 * self[1][2] + n.2 * self[2][2] + self[3][2],
        )
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

impl Mul<Matrix44f> for Vector3f {
    type Output = Vector3f;

    fn mul(self, rhs: Matrix44f) -> Self::Output {
        let mut v = Vector3f(
            self.0 * rhs[0][0] + self.1 * rhs[1][0] + self.2 * rhs[2][0] + rhs[3][0],
            self.0 * rhs[0][1] + self.1 * rhs[1][1] + self.2 * rhs[2][1] + rhs[3][1],
            self.0 * rhs[0][2] + self.1 * rhs[1][2] + self.2 * rhs[2][2] + rhs[3][2],
        );
        let w = self.0 * rhs[0][3] + self.1 * rhs[1][3] + self.2 * rhs[2][3] + rhs[3][3];
        if w != 1.0 && w != 0.0 {
            v /= w
        }
        v
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

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
