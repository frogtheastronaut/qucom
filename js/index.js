/**
 * QUCOM - Javascript SDK
 * 
 * This code is licensed under the MIT license.
 */

let init, wasm;

if (typeof window !== 'undefined') {
  // Browser
  ({ default: init, ...wasm } = await import('../pkg/web/qucom_rs.js'));
} else {
  // Node
  ({ default: init, ...wasm } = await import('../pkg/node/qucom_rs.js'));
}

let wasmInitialized = false;

/**
 * initialize the WASM module. must be called once before using any other functions.
 * @returns {Promise<void>}
 */
export async function initialize() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

/**
 * QuantumCircuit - build and execute quantum circuits
 */
export class QuantumCircuit {
    /**
     * create a new quantum circuit
     * @param {number} nQubits - number of qubits
     */
    constructor(nQubits) {
        if (!wasmInitialized) {
            throw new Error('[QUCOM] WASM not initialized. Call initialize() first.');
        }
        this._circuit = new wasm.QuantumCircuit(nQubits);
    }

    /**
     * get the number of qubits in this circuit
     * @returns {number}
     */
    get nQubits() {
        return this._circuit.n_qubits;
    }

    // ============ single qubit gates ============

    /**
     * hadamard gate - creates superposition
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    h(qubit) {
        this._circuit.h(qubit);
        return this;
    }

    /**
     * pauli-x gate - bit flip (quantum NOT)
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    x(qubit) {
        this._circuit.x(qubit);
        return this;
    }

    /**
     * Pauli-Y gate
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    y(qubit) {
        this._circuit.y(qubit);
        return this;
    }

    /**
     * Pauli-Z gate
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    z(qubit) {
        this._circuit.z(qubit);
        return this;
    }

    /**
     * S gate
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    s(qubit) {
        this._circuit.s(qubit);
        return this;
    }

    /**
     * T gate
     * @param {number} qubit - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    t(qubit) {
        this._circuit.t(qubit);
        return this;
    }

    /**
     * apply X gate to all qubits
     * @returns {QuantumCircuit} this (for chaining)
     */
    xAll() {
        this._circuit.x_all();
        return this;
    }

    /**
     * CNOT gate
     * @param {number} control - control qubit index
     * @param {number} target - target qubit index
     * @returns {QuantumCircuit} this (for chaining)
     */
    cx(control, target) {
        this._circuit.cx(control, target);
        return this;
    }

    /**
     * Toffoli gate (ccx)
     * @param {number[]} qubits - [control1, control2, target]
     * @returns {QuantumCircuit} this (for chaining)
     */
    toffoli(qubits) {
        this._circuit.toffoli(qubits);
        return this;
    }

    /**
     * multi-controlled x gate
     * @param {number[]} controls - array of control qubit indices
     * @param {number} target - target qubit index
     * @param {number[]} ancillas - array of ancilla qubit indices (optional, for efficiency)
     * @returns {QuantumCircuit} this (for chaining)
     */
    mcx(controls, target, ancillas = []) {
        this._circuit.mcx(controls, target, ancillas);
        return this;
    }

    /**
     * multi-controlled z gate - applies z when all qubits are |1⟩
     * @returns {QuantumCircuit} this (for chaining)
     */
    mcz() {
        this._circuit.mcz();
        return this;
    }

    /**
     * measure all qubits
     * @returns {QuantumCircuit} this (for chaining)
     */
    measure() {
        this._circuit.measure();
        return this;
    }

    /**
     * measure a specific qubit to a classical bit
     * @param {number} qubitIndex - qubit to measure
     * @param {number} classicalBit - classical bit index to store result
     * @returns {QuantumCircuit} this (for chaining)
     */
    measureQubit(qubitIndex, classicalBit) {
        this._circuit.measure_qubit(qubitIndex, classicalBit);
        return this;
    }

    /**
     * apply Grover's search algorithm
     * @param {number} target - target state to search for
     * @param {number} iterations - number of Grover iterations (optional, auto-calculated if not provided)
     * @returns {QuantumCircuit} this (for chaining)
     */
    groverSearch(target, iterations = null) {
        this._circuit.grover_search(target, iterations);
        return this;
    }

    /**
     * apply Grover diffusion operator
     * @returns {QuantumCircuit} this (for chaining)
     */
    diffuser() {
        this._circuit.diffuser();
        return this;
    }

    /**
     * apply Grover oracle for specific target
     * @param {number} target - target state
     * @returns {QuantumCircuit} this (for chaining)
     */
    applyGroverOracle(target) {
        this._circuit.apply_grover_oracle(target);
        return this;
    }

    /**
     * apply Deutsch-Jozsa oracle
     * @param {boolean} isConstant - true for constant function, false for balanced
     * @returns {QuantumCircuit} this (for chaining)
     */
    djOracle(isConstant) {
        this._circuit.dj_oracle(isConstant);
        return this;
    }

    /**
     * apply parity oracle (xor of all input bits)
     * @returns {QuantumCircuit} this (for chaining)
     */
    parityOracle() {
        this._circuit.parity_oracle();
        return this;
    }

    // ============ circuit operations ============

    /**
     * execute the circuit and return measurement results
     * @returns {string[]} array of measurement outcomes (binary strings)
     */
    execute() {
        return this._circuit.execute();
    }

    /**
     * reset the circuit
     * @returns {QuantumCircuit} this (for chaining)
     */
    reset() {
        this._circuit.reset();
        return this;
    }

    /**
     * export circuit as OPENQASM 2.0
     * @returns {string} QASM representation
     */
    toQASM() {
        return this._circuit.to_qasm();
    }
}

/**
 * pre-built quantum circuits
 */
export const circuits = {
    /**
     * bell state - maximally entangled 2-qubit state
     * creates |Φ+⟩ = (|00⟩ + |11⟩)/√2
     * @returns {QuantumCircuit}
     */
    bell() {
        return new QuantumCircuit(2).h(0).cx(0, 1);
    },

    /**
     * ghz state - n-qubit maximally entangled state
     * creates (|00...0⟩ + |11...1⟩)/√2
     * @param {number} n - number of qubits
     * @returns {QuantumCircuit}
     */
    ghz(n) {
        const circuit = new QuantumCircuit(n).h(0);
        for (let i = 1; i < n; i++) {
            circuit.cx(0, i);
        }
        return circuit;
    },

    /**
     * deutsch-jozsa circuit
     * @param {number} nInputQubits - number of input qubits
     * @param {boolean} isConstant - true for constant oracle, false for balanced
     * @returns {QuantumCircuit}
     */
    deutschJozsa(nInputQubits, isConstant) {
        const circuit = new QuantumCircuit(nInputQubits + 1);
        // apply hadamard to all input qubits
        for (let i = 0; i < nInputQubits; i++) {
            circuit.h(i);
        }
        // prepare ancilla in |1⟩
        circuit.x(nInputQubits).h(nInputQubits);
        // apply oracle
        circuit.djOracle(isConstant);
        // apply hadamard to input qubits
        for (let i = 0; i < nInputQubits; i++) {
            circuit.h(i);
        }
        // measure input qubits only
        for (let i = 0; i < nInputQubits; i++) {
            circuit.measureQubit(i, i);
        }
        return circuit;
    },

    /**
     * grover's search algorithm
     * @param {number} nQubits - number of qubits
     * @param {number} target - target state to find
     * @param {number} iterations - number of iterations (optional)
     * @returns {QuantumCircuit}
     */
    grover(nQubits, target, iterations = null) {
        return new QuantumCircuit(nQubits).groverSearch(target, iterations);
    }
};

/**
 * utility functions
 */
export const utils = {
    /**
     * run circuit multiple times and collect statistics
     * @param {Function} circuitFn - function that returns a QuantumCircuit
     * @param {number} shots - number of times to run
     * @returns {Object} {counts: {}, probabilities: {}, shots: number}
     */
    runTrials(circuitFn, shots = 1000) {
        const counts = {};
        
        for (let i = 0; i < shots; i++) {
            const circuit = circuitFn();
            const result = circuit.execute()[0];
            counts[result] = (counts[result] || 0) + 1;
        }

        const probabilities = {};
        for (const [outcome, count] of Object.entries(counts)) {
            probabilities[outcome] = count / shots;
        }

        return { counts, probabilities, shots };
    },

    /**
     * convert binary string to decimal
     * @param {string} binary - binary string (e.g., "101")
     * @returns {number} decimal value
     */
    binaryToDecimal(binary) {
        return parseInt(binary, 2);
    },

    /**
     * convert decimal to binary string
     * @param {number} decimal - decimal number
     * @param {number} width - minimum width (zero-padded)
     * @returns {string} binary string
     */
    decimalToBinary(decimal, width = 0) {
        return decimal.toString(2).padStart(width, '0');
    }
};

export * from wasm;