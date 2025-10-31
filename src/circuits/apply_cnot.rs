use ndarray::Array2;
use num_complex::Complex64;

pub fn apply_cnot_state(
    state: &Array2<Complex64>,
    control: usize,
    target: usize,
    n_qubits: usize,
) -> Array2<Complex64> {
    let mut new_state = Array2::<Complex64>::zeros((1 << n_qubits, 1));

    // adjust indices so qubit 0 = leftmost bit
    let control_bit = n_qubits - 1 - control;
    let target_bit = n_qubits - 1 - target;

    for i in 0..(1 << n_qubits) {
        if ((i >> control_bit) & 1) == 1 {
            let flipped_idx = i ^ (1 << target_bit);
            new_state[[flipped_idx, 0]] = state[[i, 0]];
        } else {
            new_state[[i, 0]] = state[[i, 0]];
        }
    }

    new_state
}
