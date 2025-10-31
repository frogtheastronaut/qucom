use ndarray::{Array2};
use num_complex::Complex64;
use rand::rng;
use rand_distr::weighted::WeightedIndex;
use rand_distr::Distribution;

use crate::states::{zero};
use crate::gates::{hadamard, pauli_x};
use crate::utils::kron;

/// Create the zero state for n qubits
pub fn zero_state(n_qubits: usize) -> Array2<Complex64> {
    let mut state = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();
    for _ in 0..n_qubits {
        state = kron(&state, &zero());
    }
    state
}

/// Apply a single-qubit gate to a target qubit
pub fn apply_gate(state: &Array2<Complex64>, gate: &Array2<Complex64>, target: usize, n_qubits: usize) -> Array2<Complex64> {
    let mut op = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();
    let eye = Array2::<Complex64>::eye(2);

    for i in 0..n_qubits {
        let g = if i == target { gate } else { &eye };
        op = kron(&op, g);
    }

    op.dot(state)
}

/// Apply a controlled single-qubit gate
pub fn apply_controlled_gate(state: &Array2<Complex64>, gate: &Array2<Complex64>, control: usize, target: usize, n_qubits: usize) -> Array2<Complex64> {
    let p0 = Array2::<Complex64>::from_shape_vec((2, 2), vec![
        Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0),
    ]).unwrap();

    let p1 = Array2::<Complex64>::from_shape_vec((2, 2), vec![
        Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0),
    ]).unwrap();

    let eye = Array2::<Complex64>::eye(2);

    let mut op0 = Array2::<Complex64>::from_shape_vec((1,1), vec![Complex64::new(1.0,0.0)]).unwrap();
    let mut op1 = Array2::<Complex64>::from_shape_vec((1,1), vec![Complex64::new(1.0,0.0)]).unwrap();

    for i in 0..n_qubits {
        let g0 = if i == control { &p0 } else { &eye };
        let g1 = if i == control { &p1 } else if i == target { gate } else { &eye };

        op0 = kron(&op0, g0);
        op1 = kron(&op1, g1);
    }

    let u = &op0 + &op1;
    u.dot(state)
}

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

    /// Measure the quantum state (returns a bitstring)
    pub fn measure(&self) -> String {
        let probs: Vec<f64> = self.state.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = rng();
        let idx = dist.sample(&mut rng);
        format!("{:0width$b}", idx, width = (self.state.len() as f64).log2() as usize)
    }
}
