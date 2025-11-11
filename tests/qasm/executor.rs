use qucom_rs::circuits::QuantumCircuit;

#[test]
fn execute_simple_circuit() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).measure();
    let results = circuit.execute();
    
    assert!(!results.is_empty());
    assert!(results[0].starts_with('0') || results[0].starts_with('1'));
}

#[test]
fn from_qasm_execution() {
    let qasm = "h q[0];\nmeasure q -> c;";
    let result = QuantumCircuit::from_qasm(qasm);
    
    assert!(result.is_ok());
}

#[test]
fn bell_state_execution() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).cx(0, 1).measure();
    let results = circuit.execute();
    
    assert!(results[0] == "00" || results[0] == "11");
}
