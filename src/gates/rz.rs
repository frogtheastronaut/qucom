use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// rotation around Z-axis gate
pub fn rz(angle: f64) -> Array2<Complex64> {
    let half_angle = angle / 2.0;
    
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::from_polar(1.0, -half_angle), Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0), Complex64::from_polar(1.0, half_angle),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add Rz gate to circuit
    pub fn rz(&mut self, angle: f64, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Rz(angle, qubit_index));
        self
    }
}
