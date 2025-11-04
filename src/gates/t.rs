use ndarray::Array2;
use num_complex::Complex64;
use std::f64::consts::PI;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

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
	/// add T gate to circuit
	pub fn t(&mut self, qubit_index: usize) -> &mut Self {
		self.add_instruction(QasmInstruction::T(qubit_index));
		self
	}
}