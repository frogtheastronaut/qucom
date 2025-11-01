use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
 	/// Apply Grover diffusion operator
    pub fn diffuser(&mut self) {
        self.h_all();
		self.x_all();
        self.mcz();
        self.x_all();
        self.h_all();
    }

	/// Grover Search
	/// If iterations is None, it defaults to the optimal number of iterations
	pub fn grover_search(&mut self, target: usize, iterations: Option<usize>) {
		self.h_all();
        let n_qubits = self.n;
        let iterations = iterations.unwrap_or_else(|| {
            ((std::f64::consts::PI / 4.0) * (1 << n_qubits) as f64).sqrt().round() as usize
        });

		for _ in 0..iterations {
			self.apply_oracle(target);
			self.diffuser();
		}
	}
}