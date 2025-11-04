pub use crate::gates::apply_controlled_gate;
pub use crate::gates::apply_gate;
pub mod circuit;

use ndarray::{Array2};
use num_complex::Complex64;

use crate::states::multi_qubit::zero_state;
use crate::qasm::{QasmGenerator, QasmExecutor};
use crate::qasm::generator::QasmInstruction;

pub struct QuantumCircuit {
    pub n: usize,
    qasm_generator: QasmGenerator,
    state: Option<Array2<Complex64>>,
    executed: bool,
}

impl QuantumCircuit {
    /// create a quantum circuit with n qubits
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n: n_qubits,
            qasm_generator: QasmGenerator::new(n_qubits),
            state: None,
            executed: false,
        }
    }

    /// get the current state of the circuit
    /// only available after execution
    pub fn state(&self) -> Option<&Array2<Complex64>> {
        self.state.as_ref()
    }

    /// get mutable access to state of the circuit
    pub fn state_mut(&mut self) -> Option<&mut Array2<Complex64>> {
        self.state.as_mut()
    }

    /// export circuit to QASM string
    pub fn to_qasm(&self) -> String {
        self.qasm_generator.to_qasm()
    }

    /// execute the circuit
    pub fn execute(&mut self) -> Vec<String> {
        if self.state.is_none() {
            self.state = Some(zero_state(self.n));
        }
        
        let measurements = QasmExecutor::execute(
            self.qasm_generator.instructions(),
            self.state.as_mut().unwrap(),
            self.n,
        );
        
        self.executed = true;
        measurements
    }

    /// reset the circuit
    pub fn reset(&mut self) {
        self.qasm_generator.clear(); // clear instructions
        self.state = None; // clear state
        self.executed = false;
    }

    /// check if circuit has been executed
    pub fn is_executed(&self) -> bool {
        self.executed
    }

    // internal helper to add instructions
    // for DRY principle lovers, i'm not repeating myself. i'm VERY MUCH repeating myself
    // (i believe this is an exception to the rule)
    pub(crate) fn add_instruction(&mut self, instr: QasmInstruction) {
        self.qasm_generator.add_instruction(instr);
    }
}
