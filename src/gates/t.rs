use ndarray::Array2;
use num_complex::Complex64;
use std::f64::consts::PI;
use crate::circuits::QuantumCircuit;
use crate::gates::apply_gate;

/// T gate
pub fn t() -> Array2<Complex64> {
	Array2::from_shape_vec(
		(2, 2),
		vec![
			Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
			Complex64::new(0.0, 0.0), Complex64::new((PI/4.0).cos(), (PI/4.0).sin()),
		],
	).unwrap()
}

impl QuantumCircuit {
	/// Apply T gate
	pub fn t(&mut self, qubit_index: usize) {
		self.state = apply_gate(&self.state, &t(), qubit_index, self.n);
	}
}