use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
	/// Deutsch-Jozsa Algorithm
   pub fn deutsch_jozsa(&mut self, oracle: impl Fn(&mut QuantumCircuit)) -> String {
        let n_input = self.n - 1; // last qubit is ancilla

        for i in 0..n_input {
            self.h(i);
        }

        self.x(self.n - 1);
        self.h(self.n - 1);

        oracle(self);

        for i in 0..n_input {
            self.h(i);
        }

        let mut result = String::new();
        for i in 0..n_input {
			result.push(self.measure_qubit(i));
        }

        result
    }
}