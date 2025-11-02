use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
	/// Apply Grover's Oracle
    pub fn apply_grover_oracle(&mut self, target: usize) {
        self.state[(target, 0)] *= -1.0;
    }
    /// Apply Deutsch-Jozsa Oracle
    pub fn apply_deutsch_jozsa_oracle(&mut self, is_constant: bool) {
        if !is_constant {
            let n_input = self.n - 1; // last qubit is ancilla
            for i in 0..(1 << n_input) {
                if i % 2 == 1 { // mark half the inputs
                    for ancilla in 0..2 {
                        let index = (i << 1) | ancilla;
                        self.state[(index, 0)] *= -1.0;
                    }
                }
            }
        }
    }
}