use qucom_rs::circuits::QuantumCircuit;
use qucom_rs::visualise::{state_probabilities, plot_probabilities};

fn main() {
    // Create a 3-qubit circuit
    let mut qc = QuantumCircuit::new(3);

    // Initialize all qubits in superposition
    qc.h_all();

    // Mark the target state |101> (decimal 5)
    let target = 5;

    // Optimal number of Grover iterations
    let n_qubits = 3;
    let iterations = ((std::f64::consts::PI / 4.0) * (1 << n_qubits) as f64).sqrt().round() as usize;

    // Apply Grover iterations
    for _ in 0..iterations {
        qc.apply_oracle(target);
        qc.diffuser();
    }

    // Measure once
    let result = qc.measure();
    println!("Single measurement result: {}", result);

    // Sample multiple measurements to see probability distribution
    println!("Sampling 10 measurements:");
    for _ in 0..10 {
        println!("{}", qc.measure());
    }

    // Visualize probabilities (optional)
    let probs = state_probabilities(&qc.state);
    match plot_probabilities(&probs, "output.png") {
        Ok(_) => println!("Probability distribution plot saved."),
        Err(e) => eprintln!("Plotting error: {}", e),
    }
}
