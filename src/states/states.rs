use ndarray::Array2;
use num_complex::Complex64;

// |0⟩
pub fn zero() -> Array2<Complex64> {
    Array2::from_shape_vec((2, 1), vec![
        Complex64::new(1.0, 0.0),
        Complex64::new(0.0, 0.0),
    ]).unwrap()
}

// |1⟩
pub fn one() -> Array2<Complex64> {
    Array2::from_shape_vec((2, 1), vec![
        Complex64::new(0.0, 0.0),
        Complex64::new(1.0, 0.0),
    ]).unwrap()
}
