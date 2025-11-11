pub use crate::gates::apply_controlled_gate;
pub use crate::gates::apply_gate;
pub mod circuit;
pub mod control;

use ndarray::{Array2};
use num_complex::Complex64;

use crate::states::multi_qubit::zero_state;
use crate::qasm::{QasmGenerator, QasmExecutor, QasmParser};
use crate::qasm::generator::QasmInstruction;

pub struct QuantumCircuit {
    pub n: usize,
    qasm_generator: QasmGenerator,
    state: Option<Array2<Complex64>>,
    executed: bool,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n: n_qubits,
            qasm_generator: QasmGenerator::new(n_qubits),
            state: None,
            executed: false,
        }
    }

    pub fn state(&self) -> Option<&Array2<Complex64>> {
        self.state.as_ref()
    }

    pub fn state_mut(&mut self) -> Option<&mut Array2<Complex64>> {
        self.state.as_mut()
    }

    pub fn to_qasm(&self) -> String {
        self.qasm_generator.to_qasm()
    }

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

    pub fn reset(&mut self) {
        self.qasm_generator.clear();
        self.state = None;
        self.executed = false;
    }

    pub fn is_executed(&self) -> bool {
        self.executed
    }
    
    /// parse and execute string
    pub fn from_qasm(qasm_string: &str) -> Result<Vec<String>, String> {
        let instructions = QasmParser::parse(qasm_string)?;
        
        // infer number of qubits from instructions
        let n_qubits = Self::infer_qubit_count(&instructions);
        
        let mut circuit = Self::new(n_qubits);
        
        for instr in instructions {
            circuit.add_instruction(instr);
        }
        
        Ok(circuit.execute())
    }
    
    fn infer_qubit_count(instructions: &[QasmInstruction]) -> usize {
        let mut max_qubit = 0;
        
        for instr in instructions {
            match instr {
                QasmInstruction::H(q) | QasmInstruction::X(q) | QasmInstruction::Y(q) 
                | QasmInstruction::Z(q) | QasmInstruction::S(q, _) | QasmInstruction::T(q, _)
                | QasmInstruction::Rx(_, q) | QasmInstruction::Ry(_, q) | QasmInstruction::Rz(_, q)
                | QasmInstruction::Phase(_, q) | QasmInstruction::U(_, _, _, q) 
                | QasmInstruction::Measure(q, _) | QasmInstruction::Reset(q) | QasmInstruction::Delay(_, _, q) => {
                    max_qubit = max_qubit.max(*q);
                }
                QasmInstruction::CX(c, t) | QasmInstruction::CZ(c, t) | QasmInstruction::Swap(c, t) => {
                    max_qubit = max_qubit.max(*c).max(*t);
                }
                QasmInstruction::CCX(qs) | QasmInstruction::Barrier(qs) => {
                    for q in qs {
                        max_qubit = max_qubit.max(*q);
                    }
                }
                QasmInstruction::If(_, _, body) | QasmInstruction::While(_, _, body) 
                | QasmInstruction::For(_, _, _, body) => {
                    let inner_count = Self::infer_qubit_count(body);
                    max_qubit = max_qubit.max(inner_count.saturating_sub(1));
                }
                QasmInstruction::IfElse(_, _, if_body, else_body) => {
                    let if_count = Self::infer_qubit_count(if_body);
                    let else_count = Self::infer_qubit_count(else_body);
                    max_qubit = max_qubit.max(if_count.saturating_sub(1)).max(else_count.saturating_sub(1));
                }
                QasmInstruction::MeasureAll | QasmInstruction::ResetAll | QasmInstruction::BarrierAll => {}
            }
        }
        
        max_qubit + 1
    }

    // internal helper to add instructions
    // for DRY principle lovers, i'm not repeating myself. i'm VERY MUCH repeating myself
    // (i believe this is an exception to the rule)
    pub(crate) fn add_instruction(&mut self, instr: QasmInstruction) {
        self.qasm_generator.add_instruction(instr);
    }
}
