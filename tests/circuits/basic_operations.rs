use qucom_rs::circuits::QuantumCircuit;

#[test]
fn circuit_creation() {
    let circuit = QuantumCircuit::new(3);
    assert_eq!(circuit.n, 3);
    assert!(!circuit.is_executed());
}

#[test]
fn circuit_reset() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).cx(0, 1);
    circuit.execute();
    
    assert!(circuit.is_executed());
    circuit.reset();
    assert!(!circuit.is_executed());
}

#[test]
fn multi_gate_application() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.h(0).h(1).h(2);
    circuit.cx(0, 1).cx(1, 2);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("h q[0]"));
    assert!(qasm.contains("cx q[0], q[1]"));
}
