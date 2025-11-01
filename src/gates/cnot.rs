use crate::circuits::QuantumCircuit;
use crate::gates::apply_controlled_gate;
use crate::gates::pauli_x;

impl QuantumCircuit {
	/// Apply CNOT gate
	pub fn cx(&mut self, control_qubit: usize, target_qubit: usize) {
		self.state = apply_controlled_gate(&self.state, &pauli_x(), control_qubit, target_qubit, self.n);
	}
}