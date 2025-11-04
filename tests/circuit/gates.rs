use qucom_rs::circuits::QuantumCircuit;

#[test]
fn basic_gates_work() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).x(1).cx(0, 1).measure();
    
    let results = qc.execute();
    assert!(!results.is_empty(), "Should produce measurement results");
}

#[test]
fn circuit_execution_lifecycle() {
    let mut qc = QuantumCircuit::new(2);
    
    assert!(!qc.is_executed(), "New circuit should not be executed");
    
    qc.h(0).measure().execute();

    assert!(qc.is_executed(), "Circuit should be executed after execute()");
    assert!(qc.state().is_some(), "State should be available after execution");
}

#[test]
fn empty_circuit_execution() {
    let mut qc = QuantumCircuit::new(1);
    qc.measure();
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "0", "Empty circuit should give |0⟩");
}

#[test]
fn circuit_reset_clears_state() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1).execute();
    
    qc.reset();

    assert!(!qc.is_executed(), "Reset should clear execution flag");
    assert!(qc.state().is_none(), "Reset should clear state");
}

#[test]
fn single_qubit_circuit() {
    let mut qc = QuantumCircuit::new(1);
    qc.x(0).measure();
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "1", "X gate should flip |0⟩ to |1⟩");
}
