use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// Pauli-Y gate: Y = [[0, -i], [i, 0]]
pub fn pauli_y() -> Array2<Complex64> {
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0),
            Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add Pauli-Y gate to circuit
    pub fn y(&mut self, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Y(qubit_index));
        self
    }
}

