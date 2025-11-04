use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// add Pauli-X gate to all qubits in the circuit
    pub fn x_all(&mut self) -> &mut Self {
        for qubit in 0..self.n {
            self.x(qubit);
        }
        self
    }
}