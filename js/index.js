/*
    QUCOM - Quantum Computing SDK in JS/WASM
    A JavaScript library for quantum computing using WebAssembly.
*/

export let wasm;
let initPromise = null;

async function initWasm() {
  if (typeof process !== "undefined" && process.versions?.node) {
    wasm = await import("../pkg/node/qucom_rs.js");
  } else {
    wasm = await import("../pkg/web/qucom_rs.js");
  }
  await wasm.default();
  return wasm;
}

export async function init() {
  if (!initPromise) {
    initPromise = initWasm();
  }
  return initPromise;
}

initPromise = init();

export class QuantumCircuit { 
  constructor(nQubits) {
    if (!wasm) {
      throw new Error("WASM not initialized. Please await the module initialization before creating circuits.");
    }
    this._circuit = new wasm.QuantumCircuit(nQubits);
  }
  get nQubits() { return this._circuit.n_qubits; }
  h(q) { this._circuit.h(q); return this; }
  x(q) { this._circuit.x(q); return this; }
  y(q) { this._circuit.y(q); return this; }
  z(q) { this._circuit.z(q); return this; }
  s(q) { this._circuit.s(q); return this; }
  sdg(q) { this._circuit.sdg(q); return this; }
  t(q) { this._circuit.t(q); return this; }
  tdg(q) { this._circuit.tdg(q); return this; }
  rx(angle, q) { this._circuit.rx(angle, q); return this; }
  ry(angle, q) { this._circuit.ry(angle, q); return this; }
  rz(angle, q) { this._circuit.rz(angle, q); return this; }
  phase(angle, q) { this._circuit.phase(angle, q); return this; }
  u(theta, phi, lambda, q) { this._circuit.u(theta, phi, lambda, q); return this; }
  cx(c, t) { this._circuit.cx(c, t); return this; }
  cz(c, t) { this._circuit.cz(c, t); return this; }
  swap(q1, q2) { this._circuit.swap(q1, q2); return this; }
  toffoli(qs) { this._circuit.toffoli(qs); return this; }
  mcx(ctrls, tgt, anc=[]) { this._circuit.mcx(ctrls, tgt, anc); return this; }
  mcz() { this._circuit.mcz(); return this; }
  xAll() { this._circuit.x_all(); return this; }
  measure() { this._circuit.measure(); return this; }
  measureQubit(q, c) { this._circuit.measure_qubit(q, c); return this; }
  resetQubit(q) { this._circuit.reset_qubit(q); return this; }
  resetAllQubits() { this._circuit.reset_all_qubits(); return this; }
  barrier(qs) { this._circuit.barrier(qs); return this; }
  barrierAll() { this._circuit.barrier_all(); return this; }
  execute() { return this._circuit.execute(); }
  reset() { this._circuit.reset(); return this; }
  isExecuted() { return this._circuit.is_executed(); }
  toQASM() { return this._circuit.to_qasm(); }
  static fromQASM(qasmString) { return wasm.QuantumCircuit.from_qasm(qasmString); }
}

export function getVersion() {
  if (!wasm) {
    throw new Error("WASM not initialized.");
  }
  return wasm.version();
}

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
  superposition(n) {
    const circuit = new QuantumCircuit(n);
    for (let i = 0; i < n; i++) {
      circuit.h(i);
    }
    return circuit;
  },
  qft(n) {
    const circuit = new QuantumCircuit(n);
    for (let i = 0; i < n; i++) {
      circuit.h(i);
      for (let j = i + 1; j < n; j++) {
        const angle = Math.PI / Math.pow(2, j - i);
        circuit.cx(j, i).phase(angle, i).cx(j, i);
      }
    }
    for (let i = 0; i < Math.floor(n / 2); i++) {
      circuit.swap(i, n - i - 1);
    }
    return circuit;
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
