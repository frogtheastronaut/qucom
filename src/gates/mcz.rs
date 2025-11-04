use crate::circuits::QuantumCircuit;
use crate::qasm::generator::QasmInstruction;

impl QuantumCircuit {
    /// Multi-Controlled Z gate on all qubits
    pub fn mcz(&mut self) -> &mut Self {
        let n = self.n;
        if n < 2 {
            return self; // nothing to do
        }

        if n == 2 {
            // just cz qubits 0 and 1
            self.add_instruction(QasmInstruction::CZ(0, 1));
            return self;
        }

        // MCZ = H(target) * MCX(all controls, target) * H(target)
        let target = n - 1;
        let controls: Vec<usize> = (0..n - 1).collect();

        // no ancillas available in MCZ - all qubits are used
        // this means MCX will use the no-ancilla decomposition
        // do not try this at home
        let mut ancillas = vec![];

        self.add_instruction(QasmInstruction::H(target));
        self.mcx(&controls, target, &mut ancillas);
        self.add_instruction(QasmInstruction::H(target));

        self
    }
}
