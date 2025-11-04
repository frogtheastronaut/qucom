use qucom_rs::circuits::QuantumCircuit;

#[test]
fn qasm_header_and_structure() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1);
    
    let qasm = qc.to_qasm();
    
    assert!(qasm.starts_with("OPENQASM 2.0;"));
    assert!(qasm.contains("include \"qelib1.inc\";"));
    assert!(qasm.contains("qreg q[2];"));
    assert!(qasm.contains("creg c[2];"));
}

#[test]
fn qasm_gate_ordering_preserved() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).x(1).cx(0, 1).z(0);
    
    let qasm = qc.to_qasm();
    
    let h_pos = qasm.find("h q[0];").unwrap();
    let x_pos = qasm.find("x q[1];").unwrap();
    let cx_pos = qasm.find("cx q[0],q[1];").unwrap();
    let z_pos = qasm.find("z q[0];").unwrap();
    
    assert!(h_pos < x_pos && x_pos < cx_pos && cx_pos < z_pos,
            "Gates should appear in order they were added");
}

#[test]
fn qasm_empty_circuit() {
    let qc = QuantumCircuit::new(1);
    let qasm = qc.to_qasm();
    
    assert!(qasm.contains("OPENQASM 2.0;"), "Empty circuit should have valid header");
    assert!(qasm.contains("qreg q[1];"), "Should declare registers");
}

#[test]
fn qasm_unchanged_after_execution() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1);
    
    let qasm_before = qc.to_qasm();
    qc.execute();
    let qasm_after = qc.to_qasm();
    
    assert_eq!(qasm_before, qasm_after, "QASM should not change after execution");
}

#[test]
fn qasm_all_gate_types() {
    let mut qc = QuantumCircuit::new(5);
    qc.h(0).x(1).z(2).s(3).t(4);
    
    let qasm = qc.to_qasm();
    
    assert!(qasm.contains("h q[0];"));
    assert!(qasm.contains("x q[1];"));
    assert!(qasm.contains("z q[2];"));
    assert!(qasm.contains("s q[3];"));
    assert!(qasm.contains("t q[4];"));
}
