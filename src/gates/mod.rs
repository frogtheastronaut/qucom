pub mod hadamard;
pub mod pauli_x;

pub use hadamard::hadamard;
pub use pauli_x::pauli_x;

use ndarray::Array2;
use num_complex::Complex64;
use crate::utils::kron;

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