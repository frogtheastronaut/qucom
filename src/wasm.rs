use wasm_bindgen::prelude::*;
use crate::circuits::QuantumCircuit as RustQuantumCircuit;

#[wasm_bindgen]
pub struct QuantumCircuit {
    inner: RustQuantumCircuit,
}

#[wasm_bindgen]
impl QuantumCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new(n_qubits: usize) -> Result<QuantumCircuit, JsValue> {
        if n_qubits == 0 {
            return Err(JsValue::from_str("Number of qubits must be greater than 0"));
        }
        if n_qubits > 30 {
            return Err(JsValue::from_str("Number of qubits exceeds maximum (30) for simulation"));
        }
        Ok(Self {
            inner: RustQuantumCircuit::new(n_qubits),
        })
    }
    pub fn h(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.h(qubit);
        Ok(())
    }
    pub fn x(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.x(qubit);
        Ok(())
    }
    pub fn y(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.y(qubit);
        Ok(())
    }
    pub fn z(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.z(qubit);
        Ok(())
    }
    pub fn s(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.s(qubit);
        Ok(())
    }
    pub fn sdg(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.sdg(qubit);
        Ok(())
    }
    pub fn t(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.t(qubit);
        Ok(())
    }
    pub fn tdg(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.tdg(qubit);
        Ok(())
    }
    pub fn rx(&mut self, angle: f64, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.rx(angle, qubit);
        Ok(())
    }
    pub fn ry(&mut self, angle: f64, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.ry(angle, qubit);
        Ok(())
    }
    pub fn rz(&mut self, angle: f64, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.rz(angle, qubit);
        Ok(())
    }
    pub fn phase(&mut self, angle: f64, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.phase(angle, qubit);
        Ok(())
    }
    pub fn u(&mut self, theta: f64, phi: f64, lambda: f64, qubit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit)?;
        self.inner.u(theta, phi, lambda, qubit);
        Ok(())
    }
    pub fn cx(&mut self, control: usize, target: usize) -> Result<(), JsValue> {
        self.validate_qubit(control)?;
        self.validate_qubit(target)?;
        if control == target {
            return Err(JsValue::from_str("Control and target qubits must be different"));
        }
        self.inner.cx(control, target);
        Ok(())
    }
    pub fn cz(&mut self, control: usize, target: usize) -> Result<(), JsValue> {
        self.validate_qubit(control)?;
        self.validate_qubit(target)?;
        if control == target {
            return Err(JsValue::from_str("Control and target qubits must be different"));
        }
        self.inner.cz(control, target);
        Ok(())
    }
    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit1)?;
        self.validate_qubit(qubit2)?;
        if qubit1 == qubit2 {
            return Err(JsValue::from_str("SWAP qubits must be different"));
        }
        self.inner.swap(qubit1, qubit2);
        Ok(())
    }
    pub fn toffoli(&mut self, qubits: Vec<usize>) -> Result<(), JsValue> {
        if qubits.len() != 3 {
            return Err(JsValue::from_str("Toffoli gate requires exactly 3 qubits"));
        }
        for &q in &qubits {
            self.validate_qubit(q)?;
        }
        if qubits[0] == qubits[1] || qubits[1] == qubits[2] || qubits[0] == qubits[2] {
            return Err(JsValue::from_str("Toffoli qubits must be distinct"));
        }
        self.inner.toffoli(&qubits);
        Ok(())
    }
    pub fn mcx(&mut self, controls: Vec<usize>, target: usize, ancillas: Vec<usize>) -> Result<(), JsValue> {
        if controls.is_empty() {
            return Err(JsValue::from_str("MCX requires at least one control qubit"));
        }
        self.validate_qubit(target)?;
        for &c in &controls {
            self.validate_qubit(c)?;
            if c == target {
                return Err(JsValue::from_str("Control qubits cannot be the same as target"));
            }
        }
        for &a in &ancillas {
            self.validate_qubit(a)?;
            if a == target {
                return Err(JsValue::from_str("Ancilla qubits cannot be the same as target"));
            }
        }
        let mut ancilla_vec = ancillas;
        self.inner.mcx(&controls, target, &mut ancilla_vec);
        Ok(())
    }
    pub fn mcz(&mut self) -> Result<(), JsValue> {
        self.inner.mcz();
        Ok(())
    }
    pub fn x_all(&mut self) -> Result<(), JsValue> {
        self.inner.x_all();
        Ok(())
    }
    pub fn measure(&mut self) -> Result<(), JsValue> {
        self.inner.measure();
        Ok(())
    }
    pub fn measure_qubit(&mut self, qubit_index: usize, classical_bit: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit_index)?;
        self.inner.measure_qubit(qubit_index, classical_bit);
        Ok(())
    }
    pub fn reset_qubit(&mut self, qubit_index: usize) -> Result<(), JsValue> {
        self.validate_qubit(qubit_index)?;
        self.inner.reset_qubit(qubit_index);
        Ok(())
    }
    pub fn reset_all_qubits(&mut self) -> Result<(), JsValue> {
        self.inner.reset_all_qubits();
        Ok(())
    }
    pub fn barrier(&mut self, qubits: Vec<usize>) -> Result<(), JsValue> {
        for &q in &qubits {
            self.validate_qubit(q)?;
        }
        self.inner.barrier(&qubits);
        Ok(())
    }
    pub fn barrier_all(&mut self) -> Result<(), JsValue> {
        self.inner.barrier_all();
        Ok(())
    }
    pub fn to_qasm(&self) -> String {
        self.inner.to_qasm()
    }
    pub fn execute(&mut self) -> Vec<String> {
        self.inner.execute()
    }
    pub fn reset(&mut self) {
        self.inner.reset();
    }
    pub fn is_executed(&self) -> bool {
        self.inner.is_executed()
    }
    #[wasm_bindgen(getter)]
    pub fn n_qubits(&self) -> usize {
        self.inner.n
    }
    pub fn from_qasm(qasm_string: &str) -> Result<Vec<String>, JsValue> {
        RustQuantumCircuit::from_qasm(qasm_string)
            .map_err(|e| JsValue::from_str(&e))
    }
    fn validate_qubit(&self, qubit: usize) -> Result<(), JsValue> {
        if qubit >= self.inner.n {
            Err(JsValue::from_str(&format!(
                "Qubit index {} out of bounds (circuit has {} qubits)",
                qubit, self.inner.n
            )))
        } else {
            Ok(())
        }
    }
}
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
#[wasm_bindgen(start)]
pub fn main() {
}
