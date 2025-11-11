use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// rotation around Y-axis gate
pub fn ry(angle: f64) -> Array2<Complex64> {
    let half_angle = angle / 2.0;
    let cos = half_angle.cos();
    let sin = half_angle.sin();
    
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(cos, 0.0), Complex64::new(-sin, 0.0),
            Complex64::new(sin, 0.0), Complex64::new(cos, 0.0),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add Ry gate to circuit
    pub fn ry(&mut self, angle: f64, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Ry(angle, qubit_index));
        self
    }
}
