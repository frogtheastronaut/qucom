use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
	/// Apply Oracle
    pub fn apply_oracle(&mut self, target: usize) {
        self.state[(target, 0)] *= -1.0;
    }

	/// Grover Search
	pub fn grover_search(&mut self, target: usize, iterations: usize) {
		for _ in 0..iterations {
			self.apply_oracle(target);
			self.diffuser();
		}
	}
}