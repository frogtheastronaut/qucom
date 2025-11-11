pub mod hadamard;
pub mod pauli_x;
pub mod pauli_y;
pub mod pauli_z;
pub mod s;
pub mod t;
pub mod cnot;
pub mod mcz;
pub mod x_all;
pub mod toffoli;
pub mod mcx;
pub mod rx;
pub mod ry;
pub mod rz;
pub mod phase;
pub mod u;
pub mod swap;

pub use hadamard::hadamard;
pub use pauli_x::pauli_x;
pub use pauli_y::pauli_y;
pub use pauli_z::pauli_z;
pub use s::{s, sdg};
pub use t::{t, tdg};
pub use toffoli::toffoli;
pub use rx::rx;
pub use ry::ry;
pub use rz::rz;
pub use phase::phase;
pub use u::u_gate;
pub use swap::{swap, apply_swap};


use ndarray::Array2;
use num_complex::Complex64;
use crate::utils::kron;

/// apply gate
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

/// apply a controlled gate
/// applies gate to target qubit when control qubit is |1⟩
pub fn apply_controlled_gate(
    state: &Array2<Complex64>,
    gate: &Array2<Complex64>,
    control: usize,
    target: usize,
    n_qubits: usize,
) -> Array2<Complex64> {
    let dim = 1 << n_qubits;
    let mut new_state = state.clone();

    let control_mask = 1 << (n_qubits - 1 - control);
    let target_mask = 1 << (n_qubits - 1 - target);

    for i in 0..dim {
        // only process when control bit is 1
        if (i & control_mask) != 0 {
            let target_bit = (i & target_mask) != 0;
            
            // pair this basis state with its target-flipped partner
            let j = i ^ target_mask;
            
            // skip if we've already processed this pair (process each pair once)
            if i < j {
                continue;
            }
            
            // determine which index has target=0 and which has target=1
            let (idx_0, idx_1) = if target_bit { (j, i) } else { (i, j) };
            
            let amp_0 = state[[idx_0, 0]];
            let amp_1 = state[[idx_1, 0]];

            // apply gate: |0⟩ and |1⟩ are the computational basis for the target qubit
            new_state[[idx_0, 0]] = gate[[0, 0]] * amp_0 + gate[[0, 1]] * amp_1;
            new_state[[idx_1, 0]] = gate[[1, 0]] * amp_0 + gate[[1, 1]] * amp_1;
        }
    }

    new_state
}
