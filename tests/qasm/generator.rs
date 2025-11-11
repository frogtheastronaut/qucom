use qucom_rs::circuits::QuantumCircuit;

#[test]
fn qasm_generation_basic() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).cx(0, 1);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("OPENQASM 3.0"));
    assert!(qasm.contains("qubit[2] q"));
    assert!(qasm.contains("h q[0]"));
    assert!(qasm.contains("cx q[0], q[1]"));
}

#[test]
fn qasm_generation_rotation_gates() {
    use std::f64::consts::PI;
    
    let mut circuit = QuantumCircuit::new(1);
    circuit.rx(PI / 2.0, 0).ry(PI / 4.0, 0).rz(PI, 0);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("rx("));
    assert!(qasm.contains("ry("));
    assert!(qasm.contains("rz("));
}

#[test]
fn qasm_generation_measurement() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).measure_qubit(0, 0);
    
    let qasm = circuit.to_qasm();
    assert!(qasm.contains("measure q[0] -> c[0]"));
}
