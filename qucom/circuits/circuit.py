import numpy as np
from qucom.states import ZERO, ONE
from qucom.gates import HADAMARD, PAULI_X

def zero_state(n_qubits):
    state = np.array([[1]], dtype=complex)
    for _ in range(n_qubits):
        state = np.kron(state, ZERO)
    return state

def apply_gate(state, gate, target, n_qubits):
    op = np.array([[1]], dtype=complex)
    for i in range(n_qubits):
        op = np.kron(op, gate if i == target else np.eye(2, dtype=complex))
    return op @ state

def apply_controlled_gate(state, gate, control, target, n_qubits):
    P0 = np.array([[1,0],[0,0]], dtype=complex)
    P1 = np.array([[0,0],[0,1]], dtype=complex)
    I  = np.eye(2, dtype=complex)

    op0, op1 = np.array([[1]], dtype=complex), np.array([[1]], dtype=complex)
    for i in range(n_qubits):
        op0 = np.kron(op0, P0 if i == control else I)
        op1 = np.kron(op1, P1 if i == control else (gate if i == target else I))

    U = op0 + op1
    return U @ state

class QuantumCircuit:
    def __init__(self, n_qubits):
        self.n = n_qubits
        self.state = zero_state(n_qubits)
        
    # Apply Hadamard gate to a specific qubit
    def h(self, qubit_index):
        self.state = apply_gate(self.state, HADAMARD, qubit_index, self.n)

    # Apply Pauli-X gate to a specific qubit
    def x(self, qubit_index):
        self.state = apply_gate(self.state, PAULI_X, qubit_index, self.n)

    def measure(self):
            probs = np.abs(self.state.flatten())**2
            idx = np.random.choice(len(probs), p=probs)
            return format(idx, f'0{int(np.log2(len(probs)))}b')
