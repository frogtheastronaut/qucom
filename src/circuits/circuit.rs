use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

impl QuantumCircuit {
    /// add a measurement instruction for a specific qubit
    pub fn measure_qubit(&mut self, qubit_index: usize, classical_bit: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Measure(qubit_index, classical_bit));
        self
    }

    /// add a measurement instruction for all qubits
    pub fn measure(&mut self) -> &mut Self {
        self.add_instruction(QasmInstruction::MeasureAll);
        self
    }
}
