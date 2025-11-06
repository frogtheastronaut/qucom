/*
    QUCOM - Quantum Computing SDK in JS/WASM
    A JavaScript library for quantum computing using WebAssembly.
*/
const isNode = typeof process !== "undefined" && process.versions?.node;

if (isNode) {
  wasm = await import("../pkg/node/qucom_rs.js");
} else {
  wasm = await import("../pkg/web/qucom_rs.js");
}


import * as wasm from '../pkg/qucom_rs.js';

export class QuantumCircuit {
  constructor(nQubits) {
    this._circuit = new wasm.QuantumCircuit(nQubits);
  }
  get nQubits() { return this._circuit.n_qubits; }
  h(q) { this._circuit.h(q); return this; }
  x(q) { this._circuit.x(q); return this; }
  y(q) { this._circuit.y(q); return this; }
  z(q) { this._circuit.z(q); return this; }
  s(q) { this._circuit.s(q); return this; }
  t(q) { this._circuit.t(q); return this; }
  xAll() { this._circuit.x_all(); return this; }
  cx(c, t) { this._circuit.cx(c, t); return this; }
  toffoli(qs) { this._circuit.toffoli(qs); return this; }
  mcx(ctrls, tgt, anc=[]) { this._circuit.mcx(ctrls, tgt, anc); return this; }
  mcz() { this._circuit.mcz(); return this; }
  measure() { this._circuit.measure(); return this; }
  measureQubit(q, c) { this._circuit.measure_qubit(q, c); return this; }
  groverSearch(target, iter=null) { this._circuit.grover_search(target, iter); return this; }
  diffuser() { this._circuit.diffuser(); return this; }
  applyGroverOracle(target) { this._circuit.apply_grover_oracle(target); return this; }
  djOracle(isConst) { this._circuit.dj_oracle(isConst); return this; }
  parityOracle() { this._circuit.parity_oracle(); return this; }
  execute() { return this._circuit.execute(); }
  reset() { this._circuit.reset(); return this; }
  toQASM() { return this._circuit.to_qasm(); }
}

// pre-built quantum circuits
export const circuits = {
  bell() {
    return new QuantumCircuit(2).h(0).cx(0, 1);
  },
  ghz(n) {
    const circuit = new QuantumCircuit(n).h(0);
    for (let i = 1; i < n; i++) {
      circuit.cx(0, i);
    }
    return circuit;
  },
  deutschJozsa(nInputQubits, isConstant) {
    const circuit = new QuantumCircuit(nInputQubits + 1);
    for (let i = 0; i < nInputQubits; i++) {
      circuit.h(i);
    }
    circuit.x(nInputQubits).h(nInputQubits);
    circuit.djOracle(isConstant);
    for (let i = 0; i < nInputQubits; i++) {
      circuit.h(i);
    }
    for (let i = 0; i < nInputQubits; i++) {
      circuit.measureQubit(i, i);
    }
    return circuit;
  },
  grover(nQubits, target, iterations = null) {
    return new QuantumCircuit(nQubits).groverSearch(target, iterations);
  }
};

// utility functions
export const utils = {
  runTrials(circuitFn, shots = 1000) {
    const counts = {};
    for (let i = 0; i < shots; i++) {
      const circuit = circuitFn();
      circuit.measure();
      const result = circuit.execute()[0];
      counts[result] = (counts[result] || 0) + 1;
    }
    const probabilities = {};
    for (const [outcome, count] of Object.entries(counts)) {
      probabilities[outcome] = count / shots;
    }
    return { counts, probabilities, shots };
  },
  binaryToDecimal(binary) {
    return parseInt(binary, 2);
  },
  decimalToBinary(decimal, width = 0) {
    return decimal.toString(2).padStart(width, '0');
  }
};

export { wasm };