use qucom_rs::circuits::QuantumCircuit;

#[test]
fn toffoli_gate() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.x(0).x(1).toffoli(&[0, 1, 2]).measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "111");
}

#[test]
fn mcx_with_two_controls() {
    let mut circuit = QuantumCircuit::new(3);
    circuit.x(0).x(1);
    circuit.mcx(&[0, 1], 2, &mut vec![]);
    circuit.measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "111");
}

#[test]
fn mcx_with_ancilla() {
    let mut circuit = QuantumCircuit::new(5);
    circuit.x(0).x(1).x(2);
    circuit.mcx(&[0, 1, 2], 3, &mut vec![4]);
    circuit.measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    // Check that target qubit (3) is flipped
    assert_eq!(results[0].chars().nth(3).unwrap(), '1');
}
