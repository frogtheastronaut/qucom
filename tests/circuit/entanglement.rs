use qucom_rs::circuits::QuantumCircuit;

#[test]
fn bell_state_entanglement() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1).measure();
    
    let results = qc.execute();
    let result = results.first().unwrap();
    assert!(result == "00" || result == "11",
            "Bell state should only give 00 or 11, got: {}", result);
}

#[test]
fn ghz_state_multipartite_entanglement() {
    let mut qc = QuantumCircuit::new(3);
    qc.h(0).cx(0, 1).cx(0, 2).measure();
    
    let results = qc.execute();
    let result = results.first().unwrap();
    
    assert!(result == "000" || result == "111",
            "GHZ state should only give 000 or 111, got: {}", result);
}

#[test]
fn multiple_cnots_propagation() {
    let mut qc = QuantumCircuit::new(4);
    qc.x(0).cx(0, 1).cx(0, 2).cx(0, 3).measure();
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "1111", "CNOTs should propagate |1⟩");
}

#[test]
fn double_x_cancellation() {
    let mut qc = QuantumCircuit::new(1);
    qc.x(0).x(0).measure(); // two X gates should cancel
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "0", "X·X should equal identity");
}

#[test]
fn superposition_measurement_validity() {
    let mut qc = QuantumCircuit::new(2);
    qc.h_multi(0..3).measure();
    
    let results = qc.execute();
    let result = results.first().unwrap();
    
    assert_eq!(result.len(), 2, "Should measure 2 qubits");
    assert!(result.chars().all(|c| c == '0' || c == '1'),
            "Result should only contain 0 and 1");
}
