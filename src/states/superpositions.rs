use ndarray::Array2;
use num_complex::Complex64;
use std::f64;
use crate::states::{zero, one};

pub fn psi() -> Array2<Complex64> {
    let sqrt2_inv = 1.0 / f64::consts::SQRT_2;
    &(&zero() + &one()) * Complex64::new(sqrt2_inv, 0.0)
}
