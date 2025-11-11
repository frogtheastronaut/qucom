use qucom_rs::circuits::QuantumCircuit;
use qucom_rs::qasm::generator::QasmInstruction;

#[test]
fn barrier() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.h(0).barrier(&[0, 1]).x(1);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("barrier"));
}

#[test]
fn reset_qubit() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.x(0).reset_qubit(0);
    let qasm = circuit.to_qasm();
    
    assert!(qasm.contains("reset q[0]"));
}

#[test]
fn conditional_gate() {
    let mut circuit = QuantumCircuit::new(2);
    let condition_body = vec![QasmInstruction::X(1)];
    circuit.measure_qubit(0, 0).if_eq(0, 1, condition_body);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("if"));
}
