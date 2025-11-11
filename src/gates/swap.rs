use ndarray::Array2;
use num_complex::Complex64;
use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

/// swap gate
pub fn swap() -> Array2<Complex64> {
    let mut mat = Array2::<Complex64>::eye(4);

    // Swap the |01⟩ and |10⟩ amplitudes
    mat[[1, 1]] = Complex64::new(0.0, 0.0);
    mat[[2, 2]] = Complex64::new(0.0, 0.0);
    mat[[1, 2]] = Complex64::new(1.0, 0.0);
    mat[[2, 1]] = Complex64::new(1.0, 0.0);

    mat
}


/// apply swap gate to state
pub fn apply_swap(state: &mut Array2<Complex64>, q1: usize, q2: usize, n_qubits: usize) {
    let dim = 1 << n_qubits;
    let mask1 = 1 << (n_qubits - 1 - q1);
    let mask2 = 1 << (n_qubits - 1 - q2);
    
    for i in 0..dim {
        let bit1 = (i & mask1) != 0;
        let bit2 = (i & mask2) != 0;
        
        if bit1 != bit2 {
            let j = i ^ mask1 ^ mask2;
            
            if i < j {
                let temp = state[[i, 0]];
                state[[i, 0]] = state[[j, 0]];
                state[[j, 0]] = temp;
            }
        }
    }
}

impl QuantumCircuit {
    /// add swap gate to circuit
    pub fn swap(&mut self, q1: usize, q2: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Swap(q1, q2));
        self
    }
}
