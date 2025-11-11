use qucom_rs::circuits::QuantumCircuit;

#[test]
fn cnot_gate() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.x(1).cx(1, 0).measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "11");
}

#[test]
fn cz_gate() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.h(0).h(1).cz(0, 1).measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 2);
}

#[test]
fn swap_gate() {
    let mut circuit = QuantumCircuit::new(2);
    circuit.x(0).swap(0, 1).measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "01");
}
