pub use crate::gates::apply_controlled_gate;
pub use crate::gates::apply_gate;
pub mod circuit;

use ndarray::{Array2};
use num_complex::Complex64;

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
}
