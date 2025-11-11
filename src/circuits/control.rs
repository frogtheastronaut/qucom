use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

impl QuantumCircuit {

    // reset the qubit at qubit_index to |0>
    pub fn reset_qubit(&mut self, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Reset(qubit_index));
        self
    }
    
    /// reset all qubits to |0>
    pub fn reset_all_qubits(&mut self) -> &mut Self {
        self.add_instruction(QasmInstruction::ResetAll);
        self
    }
    
    /// apply a barrier to the specified qubits
    pub fn barrier(&mut self, qubits: &[usize]) -> &mut Self {
        self.add_instruction(QasmInstruction::Barrier(qubits.to_vec()));
        self
    }
    
    /// apply a barrier to all qubits
    pub fn barrier_all(&mut self) -> &mut Self {
        self.add_instruction(QasmInstruction::BarrierAll);
        self
    }
    
    /// apply a delay to a qubit for a specified duration and unit
    pub fn delay(&mut self, duration: f64, unit: &str, qubit_index: usize) -> &mut Self {
        self.add_instruction(QasmInstruction::Delay(duration, unit.to_string(), qubit_index));
        self
    }
    
    /// if statement
    pub fn if_eq(&mut self, classical_bit: usize, value: usize, instructions: Vec<QasmInstruction>) -> &mut Self {
        self.add_instruction(QasmInstruction::If(classical_bit, value, instructions));
        self
    }
    
    /// if-else statement
    pub fn if_else(&mut self, classical_bit: usize, value: usize, if_block: Vec<QasmInstruction>, else_block: Vec<QasmInstruction>) -> &mut Self {
        self.add_instruction(QasmInstruction::IfElse(classical_bit, value, if_block, else_block));
        self
    }
    
    /// while loop
    pub fn while_eq(&mut self, classical_bit: usize, value: usize, body: Vec<QasmInstruction>) -> &mut Self {
        self.add_instruction(QasmInstruction::While(classical_bit, value, body));
        self
    }
    
    /// for loop
    pub fn for_loop(&mut self, var_name: &str, start: usize, end: usize, body: Vec<QasmInstruction>) -> &mut Self {
        self.add_instruction(QasmInstruction::For(var_name.to_string(), start, end, body));
        self
    }
}

