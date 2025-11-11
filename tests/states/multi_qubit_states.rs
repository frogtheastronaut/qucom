use qucom_rs::states::{zero_state, basis_state, superposition};
use num_complex::Complex64;
use crate::assert_float_eq;

#[test]
fn zero_state_creation() {
    let state = zero_state(2);
    assert_eq!(state.dim(), (4, 1));
    assert_eq!(state[[0, 0]], Complex64::new(1.0, 0.0));
}

#[test]
fn basis_state_from_bitstring() {
    let state = basis_state("10");
    assert_eq!(state.dim(), (4, 1));
    assert_eq!(state[[2, 0]], Complex64::new(1.0, 0.0));
}

#[test]
fn superposition_state() {
    let state = superposition(&["00", "11"]);
    let norm: f64 = state.iter().map(|c| c.norm_sqr()).sum();
    
    assert_float_eq(norm, 1.0, 1e-10);
}
