use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// universal single-qubit gate U(θ, φ, λ)
pub fn u_gate(theta: f64, phi: f64, lambda: f64) -> Array2<Complex64> {
    let half_theta = theta / 2.0;
    let cos = half_theta.cos();
    let sin = half_theta.sin();
    
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(cos, 0.0),
            -Complex64::from_polar(sin, lambda),
            Complex64::from_polar(sin, phi),
            Complex64::from_polar(cos, phi + lambda),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add U gate to circuit
    pub fn u(&mut self, theta: f64, phi: f64, lambda: f64, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::U(theta, phi, lambda, qubit_index));
        self
    }
}
