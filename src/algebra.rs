/*

Transcribed from http://cosinekitty.com/raytrace/rtsource.zip.
Original written by Don Cross.
Adapted to Rust by Gordon Tyler.

*/

#![allow(non_snake_case)]

use std::f64::consts::PI;

use num_complex::Complex;

const TOLERANCE: f64 = 1.0e-8;
const TWO_PI: f64 = 2.0 * PI;

fn complex(re: f64) -> Complex<f64> {
    Complex { re, im: 0.0 }
}

fn complex2(re: f64, im: f64) -> Complex<f64> {
    Complex { re, im }
}

fn is_zero(c: Complex<f64>) -> bool {
    c.re.abs() < TOLERANCE && c.im.abs() < TOLERANCE
}

fn filter_real(c: Vec<Complex<f64>>) -> Vec<f64> {
    c.into_iter().filter(|c| c.im.abs() < TOLERANCE).map(|c| c.re).collect()
}

fn cbrt(c: Complex<f64>, n: isize) -> Complex<f64> {
    let rho = c.norm().powf(1.0 / 3.0);
    let theta = ((TWO_PI * n as f64) + c.arg()) / 3.0;
    complex2(rho * theta.cos(), rho * theta.sin())
}

pub fn solve_quadratic(a: Complex<f64>, b: Complex<f64>, c: Complex<f64>) -> Vec<Complex<f64>> {
    if is_zero(a) {
        if is_zero(b) {
            Vec::with_capacity(0)
        } else {
            vec![-c / b]
        }
    } else {
        let radicand = b * b - 4.0 * a * c;
        if is_zero(radicand) {
            vec![-b / (2.0 * a)]
        } else {
            let r = radicand.sqrt();
            let d = 2.0 * a;

            vec![(-b + r) / d, (-b - r) / d]
        }
    }
}

pub fn solve_cubic(a: Complex<f64>, b: Complex<f64>, c: Complex<f64>, d: Complex<f64>) -> Vec<Complex<f64>> {
    if is_zero(a) {
        solve_quadratic(b, c, d)
    } else {
        let b = b / a;
        let c = c / a;
        let d = d / a;

        let S = b / 3.0;
        let D = c / 3.0 - S * S;
        let E = S * S * S + (d - S * c) / 2.0;
        let F_root = (E * E + D * D * D).sqrt();
        let mut F = -F_root - E;

        if is_zero(F) {
            F = F_root - E;
        }

        (0..3)
            .into_iter()
            .map(|i| {
                let G = cbrt(F, i);
                G - D / G - S
            })
            .collect()
    }
}

pub fn solve_quartic(
    a: Complex<f64>,
    b: Complex<f64>,
    c: Complex<f64>,
    d: Complex<f64>,
    e: Complex<f64>,
) -> Vec<Complex<f64>> {
    if is_zero(a) {
        solve_cubic(b, c, d, e)
    } else {
        let b = b / a;
        let c = c / a;
        let d = d / a;
        let e = e / a;

        let b2 = b * b;
        let b3 = b * b2;
        let b4 = b * b3;

        let alpha = (-3.0 / 8.0) * b2 + c;
        let beta = b3 / 8.0 - b * c / 2.0 + d;
        let gamma = (-3.0 / 256.0) * b4 + b2 * c / 16.0 - b * d / 4.0 + e;

        let alpha2 = alpha * alpha;
        let t = -b / 4.0;

        if is_zero(beta) {
            let rad = (alpha2 - 4.0 * gamma).sqrt();
            let r1 = ((-alpha + rad) / 2.0).sqrt();
            let r2 = ((-alpha - rad) / 2.0).sqrt();

            vec![t + r1, t - r1, t + r2, t - r2]
        } else {
            let alpha3 = alpha * alpha2;
            let P = -(alpha2 / 12.0 + gamma);
            let Q = -alpha3 / 108.0 + alpha * gamma / 3.0 - beta * beta / 8.0;
            let R = -Q / 2.0 + (Q * Q / 4.0 + P * P * P / 27.0).sqrt();
            let U = cbrt(R, 0);
            let mut y = (-5.0 / 6.0) * alpha + U;

            if is_zero(U) {
                y -= cbrt(Q, 0);
            } else {
                y -= P / (3.0 * U);
            }

            let W = (alpha + 2.0 * y).sqrt();
            let r1 = (-(3.0 * alpha + 2.0 * y + 2.0 * beta / W)).sqrt();
            let r2 = (-(3.0 * alpha + 2.0 * y - 2.0 * beta / W)).sqrt();

            vec![
                t + (W - r1) / 2.0,
                t + (W + r1) / 2.0,
                t + (-W - r2) / 2.0,
                t + (-W + r2) / 2.0,
            ]
        }
    }
}

pub fn solve_quartic_f64(a: f64, b: f64, c: f64, d: f64, e: f64) -> Vec<f64> {
    filter_real(solve_quartic(
        complex(a),
        complex(b),
        complex(c),
        complex(d),
        complex(e),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_roots(known: &[Complex<f64>], found: &[Complex<f64>]) {
        const MAX_ROOTS: usize = 4;
        assert!(found.len() <= MAX_ROOTS, "num roots out of bounds: {}", found.len());

        let mut used = [false, false, false, false];
        for k in 0..found.len() {
            let mut ok = false;
            for f in 0..found.len() {
                if !used[f] && is_zero(known[k] - found[f]) {
                    ok = true;
                    used[f] = true;
                    break;
                }
            }
            if !ok {
                panic!(
                    "Solver produced incorrect root value(s)\n\
                     Known correct roots: {:?}\n\
                     Found roots: {:?}",
                    known, found
                );
            }
        }
    }

    fn validate_polynomial(order: usize, poly: &[Complex<f64>], root: Complex<f64>) {
        let mut power = complex2(1.0, 0.0);
        let mut sum = complex2(0.0, 0.0);

        for i in 0..order {
            sum += poly[i] * power;
            power *= root;
        }

        assert!(is_zero(sum), "invalid polynomial");
    }

    fn test_known_quadratic_roots(M: Complex<f64>, K: Complex<f64>, L: Complex<f64>) {
        let a = M;
        let b = -M * (K + L);
        let c = M * K * L;
        let poly = [c, b, a];

        validate_polynomial(3, &poly, K);
        validate_polynomial(3, &poly, L);

        let found = solve_quadratic(a, b, c);
        let expected_roots = if is_zero(K - L) { 1 } else { 2 };
        assert_eq!(expected_roots, found.len());

        let known = [K, L];
        check_roots(&known, &found);
    }

    fn test_known_cubic_roots(M: Complex<f64>, K: Complex<f64>, L: Complex<f64>, N: Complex<f64>) {
        let a = M;
        let b = -M * (K + L + N);
        let c = M * (K * L + N * K + N * L);
        let d = -M * K * L * N;
        let poly = [d, c, b, a];

        validate_polynomial(4, &poly, K);
        validate_polynomial(4, &poly, L);
        validate_polynomial(4, &poly, N);

        let found = solve_cubic(a, b, c, d);
        let expected_roots = 3;
        assert_eq!(expected_roots, found.len());

        let known = [K, L, N];
        check_roots(&known, &found);
    }

    fn test_known_quartic_roots(m: Complex<f64>, a: Complex<f64>, b: Complex<f64>, c: Complex<f64>, d: Complex<f64>) {
        let A = m;
        let B = -m * (a + b + c + d);
        let C = m * (a * b + c * d + (a + b) * (c + d));
        let D = -m * (c * d * (a + b) + a * b * (c + d));
        let E = m * a * b * c * d;
        let poly = [E, D, C, B, A];

        validate_polynomial(5, &poly, a);
        validate_polynomial(5, &poly, b);
        validate_polynomial(5, &poly, c);
        validate_polynomial(5, &poly, d);

        let found = solve_quartic(A, B, C, D, E);
        let expected_roots = 4;

        assert_eq!(expected_roots, found.len());

        let known = [a, b, c, d];
        check_roots(&known, &found);
    }

    #[test]
    pub fn quadratic() {
        test_known_quadratic_roots(complex2(-2.3, 4.8), complex2(3.2, -4.1), complex2(-2.5, 7.7));
        test_known_quadratic_roots(complex2(5.5, 4.4), complex2(8.2, -2.1), complex2(8.2, -2.1));
    }

    #[test]
    pub fn cubic() {
        test_known_cubic_roots(complex(1.0), complex(2.0), complex(3.0), complex(4.0));
        test_known_cubic_roots(
            complex2(-2.3, 4.8),
            complex2(3.2, -4.1),
            complex2(-2.5, 7.7),
            complex2(53.0, -23.9),
        );
    }

    #[test]
    pub fn quartic() {
        test_known_quartic_roots(complex(1.0), complex(2.0), complex(3.0), complex(4.0), complex(5.0));
        test_known_quartic_roots(complex(1.0), complex(3.2), complex(2.5), complex(53.0), complex(-8.7));
        test_known_quartic_roots(
            complex2(-2.3, 4.8),
            complex2(3.2, -4.1),
            complex2(-2.5, 7.7),
            complex2(53.0, -23.9),
            complex2(-9.2, -8.7),
        );
    }
}
