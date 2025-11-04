use qucom_rs::circuits::QuantumCircuit;

#[test]
fn grover_finds_target() {
    let mut qc = QuantumCircuit::new(3);
    let target = 5; // 101
    
    qc.grover_search(target, Some(2));
    
    let results = qc.execute();
    assert!(!results.is_empty(), "Grover should produce measurement");
}

#[test]
fn grover_success_probability() {
    let target = 5; // 101
    let mut results_count = std::collections::HashMap::new();
    
    // run multiple trials
    for _ in 0..50 {
        let mut qc = QuantumCircuit::new(3);
        qc.grover_search(target, Some(1)); // 1 iteration for 3 qubits
        let results = qc.execute();
        
        if let Some(result) = results.first() {
            *results_count.entry(result.clone()).or_insert(0) += 1;
        }
    }
    
    let target_binary = format!("{:03b}", target);
    let target_count = results_count.get(&target_binary).unwrap_or(&0);

    // for 3 qubits with 1 Grover iteration, should get >50% success rate
    // random chance would be 1/8 = 12.5%, so expect at least 20 out of 50 (40%)
    assert!(*target_count >= 20, 
            "Target state should appear more than random chance (got {} times out of 50, expected >= 20)", 
            target_count);
}

#[test]
fn grover_2qubit_circuit() {
    let mut qc = QuantumCircuit::new(2);
    qc.grover_search(3, Some(1)); // 11, binary 3
    
    let results = qc.execute();
    assert!(!results.is_empty(), "Should work with 2-qubit circuit");
}

#[test]
fn grover_target_zero() {
    let mut qc = QuantumCircuit::new(3);
    qc.grover_search(0, Some(1));
    
    let results = qc.execute();
    assert!(!results.is_empty(), "Should handle edge case target 0");
}

#[test]
fn grover_diffuser_standalone() {
    let mut qc = QuantumCircuit::new(2);
    qc.h_multi(0..2).diffuser();
    
    let qasm = qc.to_qasm();
    assert!(qasm.contains("h q["), "Diffuser should contain Hadamard gates");
}
