mod gates;
mod circuits;
mod qasm;
mod states;
mod utils;

pub fn assert_complex_eq(a: num_complex::Complex64, b: num_complex::Complex64, epsilon: f64) {
    let diff_re = (a.re - b.re).abs();
    let diff_im = (a.im - b.im).abs();
    
    if diff_re > epsilon || diff_im > epsilon {
        panic!(
            "assertion failed: complex numbers not equal\n  left: {:?}\n right: {:?}\n epsilon: {}\n diff_re: {}, diff_im: {}",
            a, b, epsilon, diff_re, diff_im
        );
    }
}

pub fn assert_float_eq(a: f64, b: f64, epsilon: f64) {
    let diff = (a - b).abs();
    
    if diff > epsilon {
        panic!(
            "assertion failed: floats not equal\n  left: {}\n right: {}\n epsilon: {}\n diff: {}",
            a, b, epsilon, diff
        );
    }
}
