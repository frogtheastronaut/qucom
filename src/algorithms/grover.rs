use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
 	/// Apply Grover diffusion operator
    pub fn diffuser(&mut self) {
        self.h_all();
        for qubit in 0..self.n {
            self.x(qubit);
        }
        self.mcz();
        for qubit in 0..self.n {
            self.x(qubit);
        }
        self.h_all();
    }
}