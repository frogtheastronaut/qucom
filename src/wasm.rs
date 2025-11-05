use wasm_bindgen::prelude::*;
use crate::circuits::QuantumCircuit as RustQuantumCircuit;

#[wasm_bindgen]
pub struct QuantumCircuit {
    inner: RustQuantumCircuit,
}

#[wasm_bindgen]
impl QuantumCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new(n_qubits: usize) -> Self {
        Self {
            inner: RustQuantumCircuit::new(n_qubits),
        }
    }

    // single qubit gates
    pub fn h(&mut self, qubit: usize) {
        self.inner.h(qubit);
    }

    pub fn x(&mut self, qubit: usize) {
        self.inner.x(qubit);
    }

    pub fn y(&mut self, qubit: usize) {
        self.inner.y(qubit);
    }

    pub fn z(&mut self, qubit: usize) {
        self.inner.z(qubit);
    }

    pub fn s(&mut self, qubit: usize) {
        self.inner.s(qubit);
    }

    pub fn t(&mut self, qubit: usize) {
        self.inner.t(qubit);
    }

    // two qubit gates
    pub fn cx(&mut self, control: usize, target: usize) {
        self.inner.cx(control, target);
    }

    // multi-qubit gates
    pub fn toffoli(&mut self, qubits: Vec<usize>) {
        self.inner.toffoli(&qubits);
    }

    pub fn mcx(&mut self, controls: Vec<usize>, target: usize, ancillas: Vec<usize>) {
        let mut ancilla_vec = ancillas;
        self.inner.mcx(&controls, target, &mut ancilla_vec);
    }

    pub fn mcz(&mut self) {
        self.inner.mcz();
    }

    pub fn x_all(&mut self) {
        self.inner.x_all();
    }

    // measurement
    pub fn measure(&mut self) {
        self.inner.measure();
    }

    pub fn measure_qubit(&mut self, qubit_index: usize, classical_bit: usize) {
        self.inner.measure_qubit(qubit_index, classical_bit);
    }

    // algorithms
    pub fn grover_search(&mut self, target: usize, iterations: Option<usize>) {
        self.inner.grover_search(target, iterations);
    }

    pub fn diffuser(&mut self) {
        self.inner.diffuser();
    }

    pub fn apply_grover_oracle(&mut self, target: usize) {
        self.inner.apply_grover_oracle(target);
    }

    // deutsch-jozsa oracles
    pub fn dj_oracle(&mut self, is_constant: bool) {
        self.inner.dj_oracle(is_constant);
    }

    pub fn parity_oracle(&mut self) {
        self.inner.parity_oracle();
    }

    // circuit operations
    pub fn to_qasm(&self) -> String {
        self.inner.to_qasm()
    }

    pub fn execute(&mut self) -> Vec<String> {
        self.inner.execute()
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    #[wasm_bindgen(getter)]
    pub fn n_qubits(&self) -> usize {
        self.inner.n
    }
}
