use ndarray::{Array2};
use num_complex::Complex64;
use rand::rng;
use rand_distr::weighted::WeightedIndex;
use rand_distr::Distribution;

use crate::gates::{hadamard, pauli_x, apply_gate, apply_controlled_gate};
use crate::states::multi_qubit::zero_state;

/// Quantum circuit struct
pub struct QuantumCircuit {
    pub n: usize,
    pub state: Array2<Complex64>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n: n_qubits,
            state: zero_state(n_qubits),
        }
    }

    /// Apply Hadamard gate
    pub fn h(&mut self, qubit_index: usize) {
        self.state = apply_gate(&self.state, &hadamard(), qubit_index, self.n);
    }

    /// Apply Pauli-X gate
    pub fn x(&mut self, qubit_index: usize) {
        self.state = apply_gate(&self.state, &pauli_x(), qubit_index, self.n);
    }

    /// Apply CNOT gate
    pub fn cx(&mut self, control: usize, target: usize) {
        let x = pauli_x();
        self.state = apply_controlled_gate(&self.state, &x, control, target, self.n);
    }

    /// Measure the quantum state (returns a bitstring)
    pub fn measure(&self) -> String {
        let probs: Vec<f64> = self.state.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = rng();
        let idx = dist.sample(&mut rng);
        format!("{:0width$b}", idx, width = (self.state.len() as f64).log2() as usize)
    }
}
