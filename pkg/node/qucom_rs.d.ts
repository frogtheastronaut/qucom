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
