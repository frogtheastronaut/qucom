use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// rotation around X-axis gate
pub fn rx(angle: f64) -> Array2<Complex64> {
    let half_angle = angle / 2.0;
    let cos = half_angle.cos();
    let sin = half_angle.sin();
    
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(cos, 0.0), Complex64::new(0.0, -sin),
            Complex64::new(0.0, -sin), Complex64::new(cos, 0.0),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add Rx gate to circuit
    pub fn rx(&mut self, angle: f64, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Rx(angle, qubit_index));
        self
    }
}
