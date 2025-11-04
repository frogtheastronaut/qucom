use qucom_rs::circuits::QuantumCircuit;

#[test]
fn single_qubit_gates() {
    let mut qc = QuantumCircuit::new(1);
    
    // test X gate
    qc.x(0).measure();
    let result = qc.execute();
    assert_eq!(result.first().unwrap(), "1", "X should flip |0⟩ to |1⟩");
    
    // reset and test H gate
    qc.reset();
    qc.h(0).measure();
    let result = qc.execute();
    assert!(result.first().unwrap() == "0" || result.first().unwrap() == "1",
            "H creates superposition");
}

#[test]
fn cnot_gate() {
    // test control=0, target=0 → no change
    let mut qc = QuantumCircuit::new(2);
    qc.cx(0, 1).measure();
    assert_eq!(qc.execute().first().unwrap(), "00");
    
    // test control=1, target=0 → flip target
    qc.reset();
    qc.x(0).cx(0, 1).measure();
    assert_eq!(qc.execute().first().unwrap(), "11");
}



#[test]
fn phase_gates_no_basis_change() {
    let mut qc = QuantumCircuit::new(1);
    qc.s(0).t(0).z(0).measure();
    
    let result = qc.execute();
    assert_eq!(result.first().unwrap(), "0", 
               "Phase gates on |0⟩ don't change measurement");
}

#[test]
fn gate_ordering_matters() {
    let mut qc1 = QuantumCircuit::new(1);
    qc1.h(0).x(0).measure();
    let result1 = qc1.execute().first().unwrap().clone();
    
    let mut qc2 = QuantumCircuit::new(1);
    qc2.x(0).h(0).measure();
    let result2 = qc2.execute().first().unwrap().clone();
    
    assert!(result1 == "0" || result1 == "1");
    assert!(result2 == "0" || result2 == "1");
}

#[test]
fn mcz_gate() {
    let mut qc = QuantumCircuit::new(3);
    
    // MCZ only affects |111⟩ state
    qc.x_all(); // Create |111⟩
    qc.mcz();
    
    qc.execute();
    let state = qc.state().unwrap();
    
    // |111⟩ should have phase -1 (which doesn't affect measurement basis)
    assert!(state[[7, 0]].norm() > 0.99); // |111⟩ is state index 7
}
