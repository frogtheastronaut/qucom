use qucom_rs::gates::*;
use num_complex::Complex64;
use crate::assert_float_eq;

#[test]
fn hadamard_gate_matrix() {
    let h = hadamard();
    let sqrt2_inv = 1.0 / 2_f64.sqrt();
    
    assert_float_eq(h[[0, 0]].re, sqrt2_inv, 1e-10);
    assert_float_eq(h[[0, 1]].re, sqrt2_inv, 1e-10);
    assert_float_eq(h[[1, 0]].re, sqrt2_inv, 1e-10);
    assert_float_eq(h[[1, 1]].re, -sqrt2_inv, 1e-10);
}

#[test]
fn pauli_gates() {
    let x = pauli_x();
    assert_eq!(x[[0, 1]], Complex64::new(1.0, 0.0));
    assert_eq!(x[[1, 0]], Complex64::new(1.0, 0.0));
    
    let y = pauli_y();
    assert_eq!(y[[0, 1]], Complex64::new(0.0, -1.0));
    assert_eq!(y[[1, 0]], Complex64::new(0.0, 1.0));
    
    let z = pauli_z();
    assert_eq!(z[[0, 0]], Complex64::new(1.0, 0.0));
    assert_eq!(z[[1, 1]], Complex64::new(-1.0, 0.0));
}

#[test]
fn rotation_gates() {
    use std::f64::consts::PI;
    
    let angle = PI / 4.0;
    let rx = rx(angle);
    let ry = ry(angle);
    let rz = rz(angle);
    
    assert!(rx[[0, 0]].norm() > 0.0);
    assert!(ry[[0, 0]].norm() > 0.0);
    assert!(rz[[0, 0]].norm() > 0.0);
}
