use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
 	/// add Grover diffusion operator to circuit
    pub fn diffuser(&mut self) -> &mut Self {
        let n = self.n;
        self.h_multi(0..n);
        self.x_all();
        self.mcz();
        self.x_all();
        self.h_multi(0..n);
        
        self
    }

	/// build Grover Search circuit
	/// if iterations is None, uses the optimal number: π/4 * sqrt(N)
	pub fn grover_search(&mut self, target: usize, iterations: Option<usize>) -> &mut Self {
        let n_qubits = self.n;
        
		// calculate optimal number of iterations
        let iterations = iterations.unwrap_or_else(|| {
            let n = 2_usize.pow(n_qubits as u32);
            ((std::f64::consts::PI / 4.0) * (n as f64).sqrt()).round() as usize
        });

		// initialize superposition
		self.h_multi(0..n_qubits);

		// apply Grover iterations
		for _ in 0..iterations {
			self.apply_grover_oracle(target);
			self.diffuser();
		}

		// measure all qubits
		self.measure();
		self
	}
}