use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// Apply Hadamard gate to all qubits
    pub fn h_all(&mut self) {
        for qubit in 0..self.n {
            self.h(qubit);
        }
    }
}