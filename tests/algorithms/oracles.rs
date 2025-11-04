use qucom_rs::circuits::QuantumCircuit;

#[test]
fn grover_oracle_gate_decomposition() {
    let mut qc = QuantumCircuit::new(3);
    let target_val = 5;

    qc.h_multi(0..3).apply_grover_oracle(target_val);

    let qasm = qc.to_qasm();

    let x_q1_count = qasm.matches("x q[1];").count();
    assert!(x_q1_count >= 2, "should have X gates on qubit 1 for bit 1=0 in target 101");

    // check that H appears on target (last qubit for MCZ)
    assert!(qasm.contains("h q[2]"), "should have H gate on last qubit for MCZ");

    // check that MCX/CX/CCX instructions exist
    assert!(qasm.contains("cx") || qasm.contains("ccx"), "should have multi-controlled gates for MCZ");
}

#[test]
fn grover_oracle_all_ones_target() {
    let mut qc = QuantumCircuit::new(3);

    qc.h_multi(0..3).apply_grover_oracle(7); // Binary 111 for 3 qubits

    let qasm = qc.to_qasm();

    // count single-qubit X gates (not part of cx/ccx)
    let x_gates: Vec<&str> = qasm.lines()
        .filter(|line| line.contains("x q[") && !line.contains("cx") && !line.contains("ccx"))
        .collect();
    
    assert_eq!(x_gates.len(), 0, "All-1s target should not have single-qubit X flips, found: {:?}", x_gates);

    // Check H and MCX sequence exists
    assert!(qasm.contains("h q[2]"), "Should have H gate on last qubit for MCZ");
    assert!(qasm.contains("cx") || qasm.contains("ccx"), "Should have MCX instructions for MCZ");
}

#[test]
fn grover_oracle_all_zeros_target() {
    let mut qc = QuantumCircuit::new(3);

    qc.h_multi(0..3).apply_grover_oracle(0); // Binary 000

    let qasm = qc.to_qasm();

    // should have X flips before and after MCZ
    let x_count = qasm.matches("x q[").count();
    assert!(x_count >= 6, "All-0s target should have X flips, got {}", x_count);
}

#[test]
fn oracle_method_chaining() {
    let mut qc = QuantumCircuit::new(3);

    qc.h_multi(0..3)
      .apply_grover_oracle(5)
      .diffuser()
      .apply_grover_oracle(5)
      .diffuser();

    let qasm = qc.to_qasm();
    assert!(qasm.contains("OPENQASM"), "Chained oracles should produce valid QASM");
    assert!(qasm.contains("h q[2]"), "H gate on last qubit should appear in MCZ decomposition");
}
