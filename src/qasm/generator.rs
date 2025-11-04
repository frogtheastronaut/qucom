use std::fmt;

#[derive(Debug, Clone)]
pub enum QasmInstruction {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    T(usize),
    CX(usize, usize),
    CZ(usize, usize),
    CCX(Vec<usize>),
    Measure(usize, usize),
    MeasureAll,
}

impl fmt::Display for QasmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QasmInstruction::H(q) => write!(f, "h q[{}];", q),
            QasmInstruction::X(q) => write!(f, "x q[{}];", q),
            QasmInstruction::Y(q) => write!(f, "y q[{}];", q),
            QasmInstruction::Z(q) => write!(f, "z q[{}];", q),
            QasmInstruction::S(q) => write!(f, "s q[{}];", q),
            QasmInstruction::T(q) => write!(f, "t q[{}];", q),
            QasmInstruction::CX(c, t) => write!(f, "cx q[{}],q[{}];", c, t),
            QasmInstruction::CCX(qs) => write!(f, "ccx q[{}],q[{}],q[{}];", qs[0], qs[1], qs[2]),
            QasmInstruction::CZ(c, t) => write!(f, "cz q[{}],q[{}];", c, t),
            QasmInstruction::Measure(q, c) => write!(f, "measure q[{}] -> c[{}];", q, c),
            QasmInstruction::MeasureAll => write!(f, "measure q -> c;"),
        }
    }
}

/// QASM generator
pub struct QasmGenerator {
    n_qubits: usize,
    instructions: Vec<QasmInstruction>,
}

impl QasmGenerator {
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n_qubits,
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: QasmInstruction) {
        self.instructions.push(instruction);
    }

    pub fn instructions(&self) -> &[QasmInstruction] {
        &self.instructions
    }

    pub fn to_qasm(&self) -> String {
        let mut qasm = String::new();
        qasm.push_str("OPENQASM 2.0;\n");
        qasm.push_str("include \"qelib1.inc\";\n\n");
        qasm.push_str(&format!("qreg q[{}];\n", self.n_qubits));
        qasm.push_str(&format!("creg c[{}];\n\n", self.n_qubits));
        
        for instr in &self.instructions {
            qasm.push_str(&format!("{}\n", instr));
        }
        
        qasm
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }
}
