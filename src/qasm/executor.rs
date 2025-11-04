use ndarray::Array2;
use num_complex::Complex64;
use crate::qasm::generator::QasmInstruction;
use crate::gates::*;
use rand::rng;
use rand_distr::weighted::WeightedIndex;
use rand_distr::Distribution;

pub struct QasmExecutor;

impl QasmExecutor {
    /// execute QASM instructions
    pub fn execute(
        instructions: &[QasmInstruction],
        state: &mut Array2<Complex64>,
        n_qubits: usize,
    ) -> Vec<String> {
        let mut measurements = Vec::new();

        for instr in instructions {
            match instr {
                QasmInstruction::H(q) => {
                    *state = apply_gate(state, &hadamard(), *q, n_qubits);
                }
                QasmInstruction::X(q) => {
                    *state = apply_gate(state, &pauli_x(), *q, n_qubits);
                }
                QasmInstruction::Y(q) => {
                    *state = apply_gate(state, &pauli_y(), *q, n_qubits);
                }
                QasmInstruction::Z(q) => {
                    *state = apply_gate(state, &pauli_z(), *q, n_qubits);
                }
                QasmInstruction::S(q) => {
                    *state = apply_gate(state, &s(), *q, n_qubits);
                }
                QasmInstruction::T(q) => {
                    *state = apply_gate(state, &t(), *q, n_qubits);
                }
                QasmInstruction::CX(control, target) => {
                    *state = apply_controlled_gate(state, &pauli_x(), *control, *target, n_qubits);
                }
                QasmInstruction::CCX(qs) => {
                    if qs.len() != 3 {
                        panic!("CCX instruction requires exactly 3 qubits");
                    }
                    let c1 = qs[0];
                    let c2 = qs[1];
                    let target = qs[2];
                    toffoli(state, c1, c2, target, n_qubits);
                }
                QasmInstruction::CZ(control, target) => {
                    *state = apply_controlled_gate(state, &pauli_z(), *control, *target, n_qubits);
                }
                QasmInstruction::Measure(q, _c) => {
                    let result = Self::measure_qubit(state, *q, n_qubits);
                    measurements.push(result);
                }
                QasmInstruction::MeasureAll => {
                    let result = Self::measure_all(state);
                    measurements.push(result);
                }
            }
        }

        measurements
    }

    /// measure a single qubit
    fn measure_qubit(state: &mut Array2<Complex64>, qubit_index: usize, n_qubits: usize) -> String {
        let dim = 1 << n_qubits;

        let mut p0 = 0.0;
        let mut p1 = 0.0;

        for i in 0..dim {
            let bit = (i >> (n_qubits - 1 - qubit_index)) & 1;
            let amp = state[[i, 0]];
            if bit == 0 { p0 += amp.norm_sqr(); }
            else { p1 += amp.norm_sqr(); }
        }

        let dist = WeightedIndex::new(&[p0, p1]).unwrap();
        let mut rng = rng();
        let outcome = dist.sample(&mut rng);

        // collapse state
        for i in 0..dim {
            let bit = (i >> (n_qubits - 1 - qubit_index)) & 1;
            if bit != outcome {
                state[[i, 0]] = Complex64::new(0.0, 0.0);
            }
        }

        // normalize
        let norm: f64 = state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if norm > 1e-10 {
            for i in 0..dim {
                state[[i, 0]] /= norm;
            }
        }

        if outcome == 0 { "0".to_string() } else { "1".to_string() }
    }

    /// measure all qubits
    fn measure_all(state: &Array2<Complex64>) -> String {
        let probs: Vec<f64> = state.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = rng();
        let idx = dist.sample(&mut rng);
        let n_qubits = (probs.len() as f64).log2() as usize;
        format!("{:0width$b}", idx, width = n_qubits)
    }
}
