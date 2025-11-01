use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
	/// Apply Oracle
    pub fn apply_oracle(&mut self, target: usize) {
        self.state[(target, 0)] *= -1.0;
    }
}