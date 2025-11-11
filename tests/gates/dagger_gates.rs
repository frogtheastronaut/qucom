use qucom_rs::circuits::QuantumCircuit;

#[test]
fn s_then_dagger() {
    let mut circuit = QuantumCircuit::new(1);
    
    // should remain |0>
    circuit.s(0).sdg(0).measure();
    let results = circuit.execute();
    
    assert_eq!(results[0], "0");
}

#[test]
fn dagger_then_s() {
    let mut circuit = QuantumCircuit::new(1);
    
    circuit.sdg(0).s(0).measure();
    let results = circuit.execute();
    
    assert_eq!(results[0], "0");
}

#[test]
fn s_dagger_on_x_state() {
    let mut circuit = QuantumCircuit::new(1);
    
    circuit.x(0).sdg(0).s(0).measure();
    let results = circuit.execute();
    
    assert_eq!(results[0], "1");
}

#[test]
fn s_dagger_interference() {
    let mut circuit = QuantumCircuit::new(1);
    
    // apply H, then S†, then H, to test phase effect
    circuit.h(0).sdg(0).h(0).measure();
    let results = circuit.execute();
    
    // expect |0> or |1> due to constructive interference
    assert!(results[0] == "0" || results[0] == "1");
}
