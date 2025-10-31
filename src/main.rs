use qucom_rs::circuits::QuantumCircuit;

fn main() {
    // 1. Create a 3-qubit circuit
    let mut qc = QuantumCircuit::new(3);
    println!("Initial state |000>:");
    println!("{:?}", qc.state);

    // 2. Apply Hadamard to qubit 0 (create superposition)
    qc.h(0);
    println!("\nAfter applying H to qubit 0:");
    println!("{:?}", qc.state);

    // 3. Apply Hadamard to qubit 1
    qc.h(1);
    println!("\nAfter applying H to qubit 1:");
    println!("{:?}", qc.state);

    // 4. Apply Hadamard to qubit 2
    qc.h(2);
    println!("\nAfter applying H to qubit 2:");
    println!("{:?}", qc.state);

    // 5. Apply CNOT with qubit 0 as control and qubit 1 as target
    qc.cx(0, 1);
    println!("\nAfter applying CNOT (control: qubit 0, target: qubit 1):");
    println!("{:?}", qc.state);

    // 6. Measure
    let result = qc.measure();
    println!("\nMeasurement result: {}", result);

    // 7. Run multiple measurements to see probabilities
    println!("\nSampling measurement 10 times:");
    for _ in 0..10 {
        println!("{}", qc.measure());
    }
}
