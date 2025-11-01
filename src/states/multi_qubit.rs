use ndarray::Array2;
use num_complex::Complex64;
use std::f64;

use crate::utils::kron;
use crate::states::{zero, one};

/// Create the zero state for n qubits
pub fn zero_state(n_qubits: usize) -> Array2<Complex64> {
    let mut state = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();
    for _ in 0..n_qubits {
        state = kron(&state, &zero());
    }
    state
}

/// Create a basis state from a bitstring
pub fn basis_state(bitstring: &str) -> Array2<Complex64> {
    let mut state = Array2::<Complex64>::from_shape_vec((1, 1), vec![Complex64::new(1.0, 0.0)]).unwrap();

    for c in bitstring.chars() {
        let qubit = if c == '0' { zero() } else { one() };
        state = kron(&state, &qubit);
    }

    state
}

/// Create a superposition from given bitstrings
pub fn superposition(bitstrings: &[&str]) -> Array2<Complex64> {
    let mut psi = Array2::<Complex64>::zeros(basis_state(bitstrings[0]).dim());

    for &b in bitstrings {
        psi = &psi + &basis_state(b);
    }

    let norm = psi.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
    &psi / Complex64::new(norm, 0.0)
}
