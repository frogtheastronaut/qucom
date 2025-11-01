use qucom_rs::circuits::QuantumCircuit;
use qucom_rs::visualise::{state_probabilities, plot_probabilities};

/// Test Grover's Search Algorithm
/// Expected result: 101 should have highest probability, the test should pass
#[test]
fn grover_search_test() {
    let mut qc = QuantumCircuit::new(3);

    qc.h_all();

    let target = 5;

    let n_qubits = 3;
    let iterations = ((std::f64::consts::PI / 4.0) * (1 << n_qubits) as f64).sqrt().round() as usize;

    for _ in 0..iterations {
        qc.apply_oracle(target);
        qc.diffuser();
    }

    let result = qc.measure();
    println!("Single measurement result: {}", result);

    println!("Sampling 10 measurements:");
    for _ in 0..10 {
        println!("{}", qc.measure());
    }

    let probs = state_probabilities(&qc.state);
    match plot_probabilities(&probs, "output.png") {
        Ok(_) => println!("Probability distribution plot saved."),
        Err(e) => eprintln!("Plotting error: {}", e),
    }
}
