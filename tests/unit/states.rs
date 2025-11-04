use qucom_rs::circuits::QuantumCircuit;

#[test]
fn initial_state_is_zero() {
    let mut qc = QuantumCircuit::new(3);
    qc.measure();
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "000", "Initial state should be |000⟩");
}

#[test]
fn state_dimension_scaling() {
    for n in 1..=4 {
        let mut qc = QuantumCircuit::new(n);
        qc.execute();
        
        let state = qc.state().unwrap();
        let expected_dim = 2_usize.pow(n as u32);
        assert_eq!(state.shape()[0], expected_dim,
                   "{} qubits should have state dimension {}", n, expected_dim);
    }
}

#[test]
fn single_qubit_state() {
    let mut qc = QuantumCircuit::new(1);
    qc.execute();
    
    let state = qc.state().unwrap();
    assert_eq!(state.shape(), &[2, 1]);
    
    // should be |0⟩: [1, 0]
    assert!((state[[0, 0]].norm() - 1.0).abs() < 1e-10);
    assert!(state[[1, 0]].norm() < 1e-10);
}

#[test]
fn max_qubit_state() {
    let n = 5;
    let mut qc = QuantumCircuit::new(n);
    qc.x_all().measure();
    
    let results = qc.execute();
    let expected = "1".repeat(n);
    assert_eq!(results.first().unwrap(), &expected,
               "All qubits flipped should give all 1s");
}

#[test]
fn partial_measurement() {
    let mut qc = QuantumCircuit::new(3);
    qc.x(0).x(2); // Prepare |101⟩
    
    qc.measure_qubit(0, 0)
      .measure_qubit(1, 1)
      .measure_qubit(2, 2);
    
    let results = qc.execute();
    
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "1", "qubit 0 should be 1");
    assert_eq!(results[1], "0", "qubit 1 should be 0");
    assert_eq!(results[2], "1", "qubit 2 should be 1");
}
