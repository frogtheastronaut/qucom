use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// Apply Multi-Controlled Z gate
    pub fn mcz(&mut self) {
        let size = 1 << self.n;
        self.state[(size - 1, 0)] *= -1.0;
    }
}
