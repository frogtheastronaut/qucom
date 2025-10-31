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

/// Apply gate
pub fn apply_gate(
    state: &Array2<Complex64>,
    gate: &Array2<Complex64>,
    qubit_index: usize,
    n_qubits: usize,
) -> Array2<Complex64> {
    let mut op = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();
    let eye = Array2::<Complex64>::eye(2);

    for j in (0..n_qubits).rev() {
        let g = if j == qubit_index { gate } else { &eye };
        op = kron(g, &op);
    }

    op.dot(state)
}

/// Apply a controlled gate
pub fn apply_controlled_gate(
    state: &Array2<Complex64>,
    gate: &Array2<Complex64>,
    control: usize,
    target: usize,
    n_qubits: usize,
) -> Array2<Complex64> {
    let dim = 1 << n_qubits;
    let mut new_state = Array2::<Complex64>::zeros((dim, 1));
    let eye = Array2::<Complex64>::eye(2);

    // Loop over basis states
    for i in 0..dim {
        let control_bit = (i >> (n_qubits - 1 - control)) & 1;
        if control_bit == 1 {
            let mut op = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();
            for j in (0..n_qubits).rev() {
                let g = if j == target { gate } else { &eye };
                op = kron(g, &op);
            }
            new_state += &op.dot(state);
            break; // full matrix applied, exit loop
        } else {
            new_state[[i, 0]] = state[[i, 0]];
        }
    }

    new_state
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
