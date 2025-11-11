use qucom_rs::utils::kron;
use ndarray::Array2;
use num_complex::Complex64;

#[test]
fn kronecker_product() {
    let a = Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0),
            Complex64::new(3.0, 0.0), Complex64::new(4.0, 0.0),
        ],
    ).unwrap();
    
    let b = Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0),
            Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
        ],
    ).unwrap();
    
    let result = kron(&a, &b);
    assert_eq!(result.dim(), (4, 4));
}