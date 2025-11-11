use std::fmt;

#[derive(Debug, Clone)]
pub enum QasmInstruction {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize, bool),
    T(usize, bool),
    CX(usize, usize),
    CZ(usize, usize),
    CCX(Vec<usize>),
    Swap(usize, usize),
    Rx(f64, usize),
    Ry(f64, usize),
    Rz(f64, usize),
    Phase(f64, usize),
    U(f64, f64, f64, usize),
    Reset(usize),
    ResetAll,
    Barrier(Vec<usize>),
    BarrierAll,
    Delay(f64, String, usize),
    Measure(usize, usize),
    MeasureAll,
    If(usize, usize, Vec<QasmInstruction>),
    IfElse(usize, usize, Vec<QasmInstruction>, Vec<QasmInstruction>),
    While(usize, usize, Vec<QasmInstruction>),
    For(String, usize, usize, Vec<QasmInstruction>),
}

impl fmt::Display for QasmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QasmInstruction::H(q) => write!(f, "h q[{}];", q),
            QasmInstruction::X(q) => write!(f, "x q[{}];", q),
            QasmInstruction::Y(q) => write!(f, "y q[{}];", q),
            QasmInstruction::Z(q) => write!(f, "z q[{}];", q),
            QasmInstruction::S(q, false) => write!(f, "s q[{}];", q),
            QasmInstruction::S(q, true) => write!(f, "sdg q[{}];", q),
            QasmInstruction::T(q, false) => write!(f, "t q[{}];", q),
            QasmInstruction::T(q, true) => write!(f, "tdg q[{}];", q),
            QasmInstruction::CX(c, t) => write!(f, "cx q[{}], q[{}];", c, t),
            QasmInstruction::CCX(qs) => write!(f, "ccx q[{}], q[{}], q[{}];", qs[0], qs[1], qs[2]),
            QasmInstruction::CZ(c, t) => write!(f, "cz q[{}], q[{}];", c, t),
            QasmInstruction::Swap(q1, q2) => write!(f, "swap q[{}], q[{}];", q1, q2),
            QasmInstruction::Rx(angle, q) => write!(f, "rx({}) q[{}];", angle, q),
            QasmInstruction::Ry(angle, q) => write!(f, "ry({}) q[{}];", angle, q),
            QasmInstruction::Rz(angle, q) => write!(f, "rz({}) q[{}];", angle, q),
            QasmInstruction::Phase(angle, q) => write!(f, "p({}) q[{}];", angle, q),
            QasmInstruction::U(theta, phi, lambda, q) => write!(f, "u({}, {}, {}) q[{}];", theta, phi, lambda, q),
            QasmInstruction::Reset(q) => write!(f, "reset q[{}];", q),
            QasmInstruction::ResetAll => write!(f, "reset q;"),
            QasmInstruction::Barrier(qs) if qs.is_empty() => write!(f, "barrier;"),
            QasmInstruction::Barrier(qs) => {
                write!(f, "barrier ")?;
                for (i, q) in qs.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "q[{}]", q)?;
                }
                write!(f, ";")
            }
            QasmInstruction::BarrierAll => write!(f, "barrier q;"),
            QasmInstruction::Delay(duration, unit, q) => write!(f, "delay[{}{}] q[{}];", duration, unit, q),
            QasmInstruction::Measure(q, c) => write!(f, "measure q[{}] -> c[{}];", q, c),
            QasmInstruction::MeasureAll => write!(f, "measure q -> c;"),
            QasmInstruction::If(bit, val, instrs) => {
                write!(f, "if (c[{}] == {}) {{ ", bit, val)?;
                for instr in instrs {
                    write!(f, "{} ", instr)?;
                }
                write!(f, "}}")
            }
            QasmInstruction::IfElse(bit, val, if_block, else_block) => {
                write!(f, "if (c[{}] == {}) {{ ", bit, val)?;
                for instr in if_block {
                    write!(f, "{} ", instr)?;
                }
                write!(f, "}} else {{ ")?;
                for instr in else_block {
                    write!(f, "{} ", instr)?;
                }
                write!(f, "}}")
            }
            QasmInstruction::While(bit, val, body) => {
                write!(f, "while (c[{}] == {}) {{ ", bit, val)?;
                for instr in body {
                    write!(f, "{} ", instr)?;
                }
                write!(f, "}}")
            }
            QasmInstruction::For(var, start, end, body) => {
                write!(f, "for {} in [{}:{}] {{ ", var, start, end)?;
                for instr in body {
                    write!(f, "{} ", instr)?;
                }
                write!(f, "}}")
            }
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
        self.to_qasm_version(3)
    }

    /// export to QASM string with specified version
    pub fn to_qasm_version(&self, version: u8) -> String {
        let mut qasm = String::new();
        
        if version == 3 {
            qasm.push_str("OPENQASM 3.0;\n");
            qasm.push_str(&format!("qubit[{}] q;\n", self.n_qubits));
            qasm.push_str(&format!("bit[{}] c;\n\n", self.n_qubits));
        } else {
            qasm.push_str("OPENQASM 2.0;\n");
            qasm.push_str("include \"qelib1.inc\";\n\n");
            qasm.push_str(&format!("qreg q[{}];\n", self.n_qubits));
            qasm.push_str(&format!("creg c[{}];\n\n", self.n_qubits));
        }
        
        for instr in &self.instructions {
            qasm.push_str(&format!("{}\n", instr));
        }
        
        qasm
    }

    pub fn clear(&mut self) {
        self.instructions.clear();
    }
}
