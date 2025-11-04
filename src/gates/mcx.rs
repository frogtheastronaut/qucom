use crate::circuits::QuantumCircuit;

impl QuantumCircuit {
    /// Multi-Controlled X gate (MCX)
    pub fn mcx(&mut self, controls: &[usize], target: usize, ancillas: &mut Vec<usize>) -> &mut Self {
        match controls.len() {
            0 => self,
            1 => { 
                self.cx(controls[0], target); 
                self 
            },
            2 => { 
                self.toffoli(&[controls[0], controls[1], target]); 
                self 
            },
            _ => {
                // use ancilla-based decomposition
                if ancillas.is_empty() {
                    // no ancillas? use no-ancilla decomposition
					// this is not recommended
                    self.mcx_no_ancilla(controls, target);
                    return self;
                }

                // linear depth decomposition (Barenco et al.)
                let n = controls.len();
                
                if n == 3 {
                    // for 3 controls with ancilla, use optimized decomposition
                    let anc = ancillas[0];
                    self.toffoli(&[controls[0], controls[1], anc]);
                    self.toffoli(&[controls[2], anc, target]);
                    self.toffoli(&[controls[0], controls[1], anc]);
                    self.toffoli(&[controls[2], anc, target]);
                } else {
                    // for n > 3, use recursive decomposition
                    let anc = ancillas.pop().unwrap();
                    
                    // first half controls -> ancilla
                    self.mcx(&controls[..n-2], anc, ancillas);
                    
                    // last two controls + ancilla -> target (3-controlled)
                    self.toffoli(&[controls[n-2], controls[n-1], anc]);
                    let mut temp_ancillas = ancillas.clone();
                    if !temp_ancillas.is_empty() {
                        self.mcx(&[controls[n-2], controls[n-1]], target, &mut temp_ancillas);
                    } else {
                        self.toffoli(&[controls[n-2], controls[n-1], target]);
                    }
                    self.toffoli(&[controls[n-2], controls[n-1], anc]);
                    
                    // uncompute first half
                    self.mcx(&controls[..n-2], anc, ancillas);
                    
                    ancillas.push(anc);
                }
                
                self
            }
        }
    }

    /// MCX without ancillas - uses Gray code decomposition
    /// Warning: exponential number of gates!
    fn mcx_no_ancilla(&mut self, controls: &[usize], target: usize) -> &mut Self {
        if controls.len() <= 2 {
            return self.mcx(controls, target, &mut vec![]);
        }
        
        // For 3+ controls without ancilla, decompose into Toffolis
        // This is a simplified version - real implementation would use proper decomposition
        let n = controls.len();
        
        // Use Gray code recursion
        self.mcx(&controls[..n-1], target, &mut vec![]);
        self.cx(controls[n-1], target);
        self.mcx(&controls[..n-1], target, &mut vec![]);
        self.cx(controls[n-1], target);
        
        self
    }
}
