use qucom_rs::states::{zero, one};
use num_complex::Complex64;

#[test]
fn zero_state() {
    let z = zero();
    assert_eq!(z[[0, 0]], Complex64::new(1.0, 0.0));
    assert_eq!(z[[1, 0]], Complex64::new(0.0, 0.0));
}

#[test]
fn one_state() {
    let o = one();
    assert_eq!(o[[0, 0]], Complex64::new(0.0, 0.0));
    assert_eq!(o[[1, 0]], Complex64::new(1.0, 0.0));
}

#[test]
fn state_dimensions() {
    let z = zero();
    let o = one();
    
    assert_eq!(z.dim(), (2, 1));
    assert_eq!(o.dim(), (2, 1));
}
