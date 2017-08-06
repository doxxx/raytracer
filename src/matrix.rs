use std::ops::{Index, IndexMut, Mul};

use vector::Vector3f;

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
            [1.0, 0.0, 0.0, v.0],
            [0.0, 1.0, 0.0, v.1],
            [0.0, 0.0, 1.0, v.2],
            [0.0, 0.0, 0.0, 1.0],
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
        let w = self.0 * rhs[0][3] + self.0 * rhs[1][3] + self.0 * rhs[2][3] + self.0 * rhs[3][3];
        if w != 1 && w != 0 {
            v /= w
        }
        v
    }
}
