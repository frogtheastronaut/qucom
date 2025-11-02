use qucom_rs::circuits::QuantumCircuit;

#[test]
fn deutsch_jozsa_test() {
    let n_qubits = 3; // 2 input + 1 ancilla

    let mut qc_const = QuantumCircuit::new(n_qubits);

    let constant_oracle = |_qc: &mut QuantumCircuit| {
		// Constant oracle does nothing
    };

    let result_const = qc_const.deutsch_jozsa(constant_oracle);

    println!("Constant oracle result: {}", result_const);
    assert!(result_const.chars().all(|c| c == '0'), "Constant oracle should return all 0s");

    let mut qc_bal = QuantumCircuit::new(n_qubits);

    // Balanced oracle: flips the last qubit for half the input states
    let balanced_oracle = |qc: &mut QuantumCircuit| {
        let n_input = qc.n - 1;
        for i in 0..(1 << (n_input)) {
            if i % 2 == 1 {
                let index = (i << 1) | 1; // flip ancilla for this input
                qc.state[(index, 0)] *= -1.0;
            }
        }
    };

    let result_bal = qc_bal.deutsch_jozsa(balanced_oracle);

    println!("Balanced oracle result: {}", result_bal);
    assert!(result_bal.chars().any(|c| c == '1'), "Balanced oracle should return at least one 1");
}
