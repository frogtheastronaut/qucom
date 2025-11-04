use qucom_rs::circuits::QuantumCircuit;

#[test]
fn deterministic_circuit_execution() {
    let mut qc = QuantumCircuit::new(2);
    qc.x(0).x(1).measure();
    
    let results = qc.execute();
    assert_eq!(results.first().unwrap(), "11",
               "Deterministic circuit should always give same result");
}

#[test]
fn measurement_collapses_superposition() {
    let mut qc = QuantumCircuit::new(1);
    qc.h(0).measure();
    
    let results = qc.execute();
    let result = results.first().unwrap();
    
    assert!(result == "0" || result == "1",
            "Measurement should collapse to definite state");
}

#[test]
fn execute_without_measurement() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1);
    
    let results = qc.execute();
    assert!(results.is_empty(), "No measurement should return empty results");
}

#[test]
fn multiple_individual_measurements() {
    let mut qc = QuantumCircuit::new(3);
    qc.x(0).x(2)
      .measure_qubit(0, 0)
      .measure_qubit(1, 1)
      .measure_qubit(2, 2);
    
    let results = qc.execute();
    
    assert_eq!(results.len(), 3, "Should have 3 measurements");
    assert_eq!(results[0], "1");
    assert_eq!(results[1], "0");
    assert_eq!(results[2], "1");
}

#[test]
fn state_access_after_execution() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0);
    
    qc.execute();
    
    let state1 = qc.state().unwrap();
    let state2 = qc.state().unwrap();
    
    assert_eq!(state1, state2, "Multiple state accesses should return same data");
    
    let total_prob = state1[[0, 0]].norm_sqr() + state1[[1, 0]].norm_sqr() 
                   + state1[[2, 0]].norm_sqr() + state1[[3, 0]].norm_sqr();
    assert!((total_prob - 1.0).abs() < 1e-10, "Total probability should be 1");
}
