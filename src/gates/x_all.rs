use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// Apply Pauli X gate to all qubits
    pub fn x_all(&mut self) {
        for qubit in 0..self.n {
            self.x(qubit);
        }
    }
}