use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

impl QuantumCircuit {
	/// add CNOT gate to circuit
	pub fn cx(&mut self, control_qubit: usize, target_qubit: usize) -> &mut Self {
		self.add_instruction(QasmInstruction::CX(control_qubit, target_qubit));
		self
	}
}