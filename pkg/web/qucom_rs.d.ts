/* tslint:disable */
/* eslint-disable */
export class QuantumCircuit {
  free(): void;
  [Symbol.dispose](): void;
  constructor(n_qubits: number);
  h(qubit: number): void;
  x(qubit: number): void;
  y(qubit: number): void;
  z(qubit: number): void;
  s(qubit: number): void;
  t(qubit: number): void;
  cx(control: number, target: number): void;
  toffoli(qubits: Uint32Array): void;
  mcx(controls: Uint32Array, target: number, ancillas: Uint32Array): void;
  mcz(): void;
  x_all(): void;
  measure(): void;
  measure_qubit(qubit_index: number, classical_bit: number): void;
  grover_search(target: number, iterations?: number | null): void;
  diffuser(): void;
  apply_grover_oracle(target: number): void;
  dj_oracle(is_constant: boolean): void;
  parity_oracle(): void;
  to_qasm(): string;
  execute(): string[];
  reset(): void;
  readonly n_qubits: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_quantumcircuit_free: (a: number, b: number) => void;
  readonly quantumcircuit_new: (a: number) => number;
  readonly quantumcircuit_h: (a: number, b: number) => void;
  readonly quantumcircuit_x: (a: number, b: number) => void;
  readonly quantumcircuit_y: (a: number, b: number) => void;
  readonly quantumcircuit_z: (a: number, b: number) => void;
  readonly quantumcircuit_s: (a: number, b: number) => void;
  readonly quantumcircuit_t: (a: number, b: number) => void;
  readonly quantumcircuit_cx: (a: number, b: number, c: number) => void;
  readonly quantumcircuit_toffoli: (a: number, b: number, c: number) => void;
  readonly quantumcircuit_mcx: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly quantumcircuit_mcz: (a: number) => void;
  readonly quantumcircuit_x_all: (a: number) => void;
  readonly quantumcircuit_measure: (a: number) => void;
  readonly quantumcircuit_measure_qubit: (a: number, b: number, c: number) => void;
  readonly quantumcircuit_grover_search: (a: number, b: number, c: number) => void;
  readonly quantumcircuit_diffuser: (a: number) => void;
  readonly quantumcircuit_apply_grover_oracle: (a: number, b: number) => void;
  readonly quantumcircuit_dj_oracle: (a: number, b: number) => void;
  readonly quantumcircuit_parity_oracle: (a: number) => void;
  readonly quantumcircuit_to_qasm: (a: number) => [number, number];
  readonly quantumcircuit_execute: (a: number) => [number, number];
  readonly quantumcircuit_reset: (a: number) => void;
  readonly quantumcircuit_n_qubits: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
