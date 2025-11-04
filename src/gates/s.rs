use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// S gate
pub fn s() -> Array2<Complex64> {
	Array2::from_shape_vec(
		(2, 2),
		vec![
			Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
			Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0),
		],
	).unwrap()
}

impl QuantumCircuit {
	/// add S gate to circuit
	pub fn s(&mut self, qubit_index: usize) -> &mut Self {
		self.add_instruction(QasmInstruction::S(qubit_index));
		self
	}
}