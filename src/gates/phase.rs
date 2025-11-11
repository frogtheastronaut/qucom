use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// phase gate
pub fn phase(angle: f64) -> Array2<Complex64> {
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0), Complex64::from_polar(1.0, angle),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add phase gate to circuit
    pub fn phase(&mut self, angle: f64, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Phase(angle, qubit_index));
        self
    }
}
