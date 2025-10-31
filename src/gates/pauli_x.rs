use ndarray::Array2;
use num_complex::Complex64;

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
