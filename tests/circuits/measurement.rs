use qucom_rs::circuits::QuantumCircuit;

#[test]
fn single_qubit_measurement() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).measure_qubit(0, 0);
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert!(results[0] == "0" || results[0] == "1");
}

#[test]
fn measure_all() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.h(0).h(1).h(2).measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 3);
}

#[test]
fn deterministic_measurement() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.x(0).x(1).measure();
    let results = circuit.execute();
    
    assert_eq!(results[0], "11");
}
