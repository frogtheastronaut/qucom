use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// implement Grover oracle that flips the phase of the target state
    pub fn apply_grover_oracle(&mut self, target: usize) -> &mut Self {
        let n = self.n;
        
        // flip qubits that should be 0 in the target state
        for i in 0..n {
            let bit = (target >> (n - 1 - i)) & 1;
            if bit == 0 {
                self.x(i);
            }
        }
        
        // apply multi-controlled Z to flip phase of |11...1⟩
        self.mcz();

        // flip qubits back to restore the basis
        for i in 0..n {
            let bit = (target >> (n - 1 - i)) & 1;
            if bit == 0 {
                self.x(i);
            }
        }
        
        self
    }

    /// Oracle for Deutsch-Jozsa algorithm
    pub fn dj_oracle(&mut self, is_constant: bool) -> &mut Self {
        if is_constant {
            // constant: flip ancilla qubit
            self.x(self.n - 1);
        } else {
            // balanced: apply CNOT from first input qubit to ancilla
            let n_input = self.n - 1;
            let ancilla = self.n - 1;
            
            if n_input > 0 {
                self.cx(0, ancilla);
            }
        }
        self
    }
    
    /// checks if the number of 1s in the input is odd
    pub fn parity_oracle(&mut self) -> &mut Self {
        let n_input = self.n - 1;
        let ancilla = self.n - 1;
        
        for i in 0..n_input {
            self.cx(i, ancilla);
        }
        
        self
    }
}