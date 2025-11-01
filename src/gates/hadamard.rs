use ndarray::Array2;
use num_complex::Complex64;
use std::f64;
use crate::circuits::QuantumCircuit;
use crate::gates::apply_gate;

/// Hadamard gate
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

impl QuantumCircuit {
    /// Apply Hadamard gate
    pub fn h(&mut self, qubit_index: usize) {
        self.state = apply_gate(&self.state, &hadamard(), qubit_index, self.n);
    }
}