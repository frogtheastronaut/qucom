use qucom_rs::circuits::QuantumCircuit;

#[test]
fn multiple_qubit_hadamard() {
    let mut circuit = QuantumCircuit::new(10);

    for i in 0..10 {
        circuit.h(i);
    }

    circuit.measure();
    let results = circuit.execute();

    // there should be 1 result string of length 10
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 10);

    for c in results[0].chars() {
        assert!(c == '0' || c == '1');
    }
}

#[test]
fn multiple_qubit_x() {
    let mut circuit = QuantumCircuit::new(10);

    for i in (0..10).step_by(2) {
        circuit.x(i);
    }

    circuit.measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 10);

    for (i, c) in results[0].chars().enumerate() {
        if i % 2 == 0 {
            assert_eq!(c, '1');
        } else {
            assert_eq!(c, '0');
        }
    }
}

#[test]
fn multiple_qubit_math() {
    let mut circuit = QuantumCircuit::new(10);

    circuit.x(0).x(1).x(2);

    circuit.measure();
    let results = circuit.execute();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].len(), 10);
}
