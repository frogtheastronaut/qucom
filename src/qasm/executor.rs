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
        Self::execute_instruction(instructions, state, n_qubits, &mut vec![0; n_qubits])
    }
    
    fn execute_instruction(
        instructions: &[QasmInstruction],
        state: &mut Array2<Complex64>,
        n_qubits: usize,
        classical_bits: &mut Vec<usize>,
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
                QasmInstruction::S(q, false) => {
                    *state = apply_gate(state, &s(), *q, n_qubits);
                }
                QasmInstruction::S(q, true) => {
                    *state = apply_gate(state, &sdg(), *q, n_qubits);
                }
                QasmInstruction::T(q, false) => {
                    *state = apply_gate(state, &t(), *q, n_qubits);
                }
                QasmInstruction::T(q, true) => {
                    *state = apply_gate(state, &tdg(), *q, n_qubits);
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
                QasmInstruction::Swap(q1, q2) => {
                    apply_swap(state, *q1, *q2, n_qubits);
                }
                QasmInstruction::Rx(angle, q) => {
                    *state = apply_gate(state, &rx(*angle), *q, n_qubits);
                }
                QasmInstruction::Ry(angle, q) => {
                    *state = apply_gate(state, &ry(*angle), *q, n_qubits);
                }
                QasmInstruction::Rz(angle, q) => {
                    *state = apply_gate(state, &rz(*angle), *q, n_qubits);
                }
                QasmInstruction::Phase(angle, q) => {
                    *state = apply_gate(state, &phase(*angle), *q, n_qubits);
                }
                QasmInstruction::U(theta, phi, lambda, q) => {
                    *state = apply_gate(state, &u_gate(*theta, *phi, *lambda), *q, n_qubits);
                }
                QasmInstruction::Reset(q) => {
                    Self::reset_qubit(state, *q, n_qubits);
                }
                QasmInstruction::ResetAll => {
                    *state = crate::states::multi_qubit::zero_state(n_qubits);
                }
                QasmInstruction::Barrier(_) | QasmInstruction::BarrierAll => {
                    // barrier has no effect on simulation
                }
                QasmInstruction::Delay(_, _, _) => {
                    // delay has no effect on ideal simulation
                }
                QasmInstruction::If(bit, value, instrs) => {
                    if *bit < classical_bits.len() && classical_bits[*bit] == *value {
                        Self::execute_instruction(instrs, state, n_qubits, classical_bits);
                    }
                }
                QasmInstruction::IfElse(bit, value, if_block, else_block) => {
                    if *bit < classical_bits.len() && classical_bits[*bit] == *value {
                        Self::execute_instruction(if_block, state, n_qubits, classical_bits);
                    } else {
                        Self::execute_instruction(else_block, state, n_qubits, classical_bits);
                    }
                }
                QasmInstruction::While(bit, value, body) => {
                    while *bit < classical_bits.len() && classical_bits[*bit] == *value {
                        Self::execute_instruction(body, state, n_qubits, classical_bits);
                    }
                }
                QasmInstruction::For(_, start, end, body) => {
                    for _ in *start..*end {
                        Self::execute_instruction(body, state, n_qubits, classical_bits);
                    }
                }
                QasmInstruction::Measure(q, c) => {
                    let result = Self::measure_qubit(state, *q, n_qubits);
                    let bit_value = if result == "1" { 1 } else { 0 };
                    if *c < classical_bits.len() {
                        classical_bits[*c] = bit_value;
                    }
                    measurements.push(result);
                }
                QasmInstruction::MeasureAll => {
                    let result = Self::measure_all(state);
                    // update classical bits
                    for (i, ch) in result.chars().enumerate() {
                        if i < classical_bits.len() {
                            classical_bits[i] = if ch == '1' { 1 } else { 0 };
                        }
                    }
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
    
    /// reset qubit to |0⟩ state
    fn reset_qubit(state: &mut Array2<Complex64>, qubit_index: usize, n_qubits: usize) {
        // measure the qubit
        let measurement = Self::measure_qubit(state, qubit_index, n_qubits);
        
        // if measured |1⟩, apply X to flip it to |0⟩
        if measurement == "1" {
            *state = apply_gate(state, &pauli_x(), qubit_index, n_qubits);
        }
    }
}
