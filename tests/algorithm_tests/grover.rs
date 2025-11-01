use qucom_rs::circuits::QuantumCircuit;
use qucom_rs::visualise::{state_probabilities, plot_probabilities};

/// Test Grover's Search Algorithm
/// Expected result: 101 should have highest probability
#[test]
fn grover_search_test() {
    let mut qc = QuantumCircuit::new(3);
    let target = 5;

    qc.grover_search(target, None);

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
