
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let WASM_VECTOR_LEN = 0;

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

const QuantumCircuitFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_quantumcircuit_free(ptr >>> 0, 1));

class QuantumCircuit {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        QuantumCircuitFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_quantumcircuit_free(ptr, 0);
    }
    /**
     * @param {number} n_qubits
     */
    constructor(n_qubits) {
        const ret = wasm.quantumcircuit_new(n_qubits);
        this.__wbg_ptr = ret >>> 0;
        QuantumCircuitFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} qubit
     */
    h(qubit) {
        wasm.quantumcircuit_h(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} qubit
     */
    x(qubit) {
        wasm.quantumcircuit_x(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} qubit
     */
    y(qubit) {
        wasm.quantumcircuit_y(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} qubit
     */
    z(qubit) {
        wasm.quantumcircuit_z(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} qubit
     */
    s(qubit) {
        wasm.quantumcircuit_s(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} qubit
     */
    t(qubit) {
        wasm.quantumcircuit_t(this.__wbg_ptr, qubit);
    }
    /**
     * @param {number} control
     * @param {number} target
     */
    cx(control, target) {
        wasm.quantumcircuit_cx(this.__wbg_ptr, control, target);
    }
    /**
     * @param {Uint32Array} qubits
     */
    toffoli(qubits) {
        const ptr0 = passArray32ToWasm0(qubits, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.quantumcircuit_toffoli(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Uint32Array} controls
     * @param {number} target
     * @param {Uint32Array} ancillas
     */
    mcx(controls, target, ancillas) {
        const ptr0 = passArray32ToWasm0(controls, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray32ToWasm0(ancillas, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.quantumcircuit_mcx(this.__wbg_ptr, ptr0, len0, target, ptr1, len1);
    }
    mcz() {
        wasm.quantumcircuit_mcz(this.__wbg_ptr);
    }
    x_all() {
        wasm.quantumcircuit_x_all(this.__wbg_ptr);
    }
    measure() {
        wasm.quantumcircuit_measure(this.__wbg_ptr);
    }
    /**
     * @param {number} qubit_index
     * @param {number} classical_bit
     */
    measure_qubit(qubit_index, classical_bit) {
        wasm.quantumcircuit_measure_qubit(this.__wbg_ptr, qubit_index, classical_bit);
    }
    /**
     * @param {number} target
     * @param {number | null} [iterations]
     */
    grover_search(target, iterations) {
        wasm.quantumcircuit_grover_search(this.__wbg_ptr, target, isLikeNone(iterations) ? 0x100000001 : (iterations) >>> 0);
    }
    diffuser() {
        wasm.quantumcircuit_diffuser(this.__wbg_ptr);
    }
    /**
     * @param {number} target
     */
    apply_grover_oracle(target) {
        wasm.quantumcircuit_apply_grover_oracle(this.__wbg_ptr, target);
    }
    /**
     * @param {boolean} is_constant
     */
    dj_oracle(is_constant) {
        wasm.quantumcircuit_dj_oracle(this.__wbg_ptr, is_constant);
    }
    parity_oracle() {
        wasm.quantumcircuit_parity_oracle(this.__wbg_ptr);
    }
    /**
     * @returns {string}
     */
    to_qasm() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.quantumcircuit_to_qasm(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string[]}
     */
    execute() {
        const ret = wasm.quantumcircuit_execute(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    reset() {
        wasm.quantumcircuit_reset(this.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    get n_qubits() {
        const ret = wasm.quantumcircuit_n_qubits(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) QuantumCircuit.prototype[Symbol.dispose] = QuantumCircuit.prototype.free;

exports.QuantumCircuit = QuantumCircuit;

exports.__wbg___wbindgen_throw_b855445ff6a94295 = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

exports.__wbg_getRandomValues_1c61fac11405ffdc = function() { return handleError(function (arg0, arg1) {
    globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
}, arguments) };

exports.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

const wasmPath = `${__dirname}/qucom_rs_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
const wasm = exports.__wasm = new WebAssembly.Instance(wasmModule, imports).exports;

wasm.__wbindgen_start();

