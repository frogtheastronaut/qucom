use ndarray::Array2;
use num_complex::Complex64;
use std::f64;

// Hadamard gate
pub fn hadamard() -> Array2<Complex64> {
    let sqrt2_inv = 1.0 / f64::consts::SQRT_2;
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(sqrt2_inv, 0.0), Complex64::new(sqrt2_inv, 0.0),
            Complex64::new(sqrt2_inv, 0.0), Complex64::new(-sqrt2_inv, 0.0),
        ],
    ).unwrap()
}

// Pauli-X gate
pub fn pauli_x() -> Array2<Complex64> {
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
        ],
    ).unwrap()
}
