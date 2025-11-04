use ndarray::Array2;
use num_complex::Complex64;
use std::f64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// Hadamard gate
pub fn hadamard() -> Array2<Complex64> {
    let sqrt2_inv = 1.0 / f64::consts::SQRT_2;
    Array2::from_shape_vec(
        (2, 2),
        vec![
            Complex64::new(sqrt2_inv, 0.0), Complex64::new(sqrt2_inv, 0.0),
            Complex64::new(sqrt2_inv, 0.0), Complex64::new(-sqrt2_inv, 0.0),
        ],
    ).unwrap()
}

impl QuantumCircuit {
    /// add Hadamard gate to circuit
    pub fn h(&mut self, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::H(qubit_index));
        self
    }
    /// add Hadamard gate to multiple qubits
    pub fn h_multi(&mut self, range: std::ops::Range<usize>) -> &mut Self {
        for qubit in range {
            self.h(qubit);
        }
        self
    }
}