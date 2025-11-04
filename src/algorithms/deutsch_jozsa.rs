use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
	/// build Deutsch-Jozsa circuit
    pub fn deutsch_jozsa(&mut self, oracle: impl Fn(&mut QuantumCircuit)) -> &mut Self {
        let n_input = self.n - 1; 

        self.h_multi(0..n_input);
        self.x(self.n - 1);
        self.h(self.n - 1);

        // apply oracle
        oracle(self);
        self.h_multi(0..n_input);

        // measure input qubits
        // note: ancilla qubit is not measured, so we cannot use measure()
        for i in 0..n_input {
            self.measure_qubit(i, i);
        }

        // return circuit for chaining
        self
    }
}