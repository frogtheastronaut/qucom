use qucom_rs::circuits::QuantumCircuit;



#[test]
fn deutsch_jozsa_constant_oracle() {
    let n_qubits = 3; // 2 input + 1 ancilla
    let mut qc = QuantumCircuit::new(n_qubits);
    
    let constant_oracle = |_circuit: &mut QuantumCircuit| {}; // Do nothing
    
    qc.deutsch_jozsa(constant_oracle);
    let results = qc.execute();
    
    assert_eq!(results.len(), n_qubits - 1, "should measure n-1 input qubits");
    assert!(results.iter().all(|r| r == "0"), 
            "constant function should give all 0s");
}

#[test]
fn deutsch_jozsa_balanced_oracle() {
    let n_qubits = 3; // 2 input + 1 ancilla
    let mut qc = QuantumCircuit::new(n_qubits);
    
    let balanced_oracle = |circuit: &mut QuantumCircuit| {
        circuit.dj_oracle(false);
    };
    
    qc.deutsch_jozsa(balanced_oracle);
    let results = qc.execute();
    
    assert_eq!(results.len(), n_qubits - 1);
    assert!(results.iter().any(|r| r == "1"),
            "balanced function should have at least one 1");
}



#[test]
fn deutsch_jozsa_minimal_circuit() {
    let mut qc = QuantumCircuit::new(2); // 1 input + 1 ancilla
    
    let balanced_oracle = |circuit: &mut QuantumCircuit| {
        circuit.dj_oracle(false);
    };
    
    qc.deutsch_jozsa(balanced_oracle);
    let results = qc.execute();
    
    assert_eq!(results.len(), 1, "should measure 1 input qubit");
}

#[test]
fn deutsch_jozsa_parity_oracle() {
    let n_qubits = 4; // 3 input + 1 ancilla
    let mut qc = QuantumCircuit::new(n_qubits);
    
    let parity_oracle = |circuit: &mut QuantumCircuit| {
        circuit.parity_oracle();
    };
    
    qc.deutsch_jozsa(parity_oracle);
    let results = qc.execute();
    
    assert_eq!(results.len(), n_qubits - 1);
    assert!(results.iter().any(|r| r == "1"),
            "parity (balanced) should give non-zero result");
}

#[test]
fn deutsch_jozsa_determinism() {
    let mut results_set = std::collections::HashSet::new();
    
    for _ in 0..10 {
        let mut qc = QuantumCircuit::new(3);
        qc.deutsch_jozsa(|_circuit| {}); // Constant oracle
        let results = qc.execute();
        
        let result_str: String = results.iter().map(|s| s.as_str()).collect();
        results_set.insert(result_str);
    }
    
    assert_eq!(results_set.len(), 1, 
               "Constant oracle should always give same result");
}
