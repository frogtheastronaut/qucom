use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::gates::apply_gate;

/// Pauli-Z gate
pub fn pauli_z() -> Array2<Complex64> {
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// Apply Pauli-Z gate
    pub fn z(&mut self, qubit_index: usize) {
        self.state = apply_gate(&self.state, &pauli_z(), qubit_index, self.n);
    }
}
