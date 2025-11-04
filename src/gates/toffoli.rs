use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// Toffoli (CCX) gate
pub fn toffoli(state: &mut Array2<Complex64>, c1: usize, c2: usize, target: usize, n_qubits: usize) {
    let size = 1 << n_qubits;
    
    let c1_mask = 1 << (n_qubits - 1 - c1);
    let c2_mask = 1 << (n_qubits - 1 - c2);
    let target_mask = 1 << (n_qubits - 1 - target);
    
    for i in 0..size {
        // check if both control bits are 1
        if (i & c1_mask) != 0 && (i & c2_mask) != 0 {
            let flipped = i ^ target_mask;
            // only swap once per pair
            if i < flipped {
                state.swap((i, 0), (flipped, 0));
            }
        }
    }
}


impl QuantumCircuit {
	/// add Toffoli gate to circuit
	pub fn toffoli(&mut self, qubit_indices: &[usize]) -> &mut Self {
		self.add_instruction(QasmInstruction::CCX(qubit_indices.to_vec()));
		self
	}
}