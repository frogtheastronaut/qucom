import numpy as np
from qucom.states import ZERO, ONE
from qucom.gates import HADAMARD as H

def zero_state(n_qubits):
    state = np.array([[1]], dtype=complex)
    for _ in range(n_qubits):
        state = np.kron(state, ZERO)
    return state

def apply_gate(state, gate, target, n_qubits):
    op = 1
    for i in range(n_qubits):
        if i == target:
            op = np.kron(op, gate) if not isinstance(op, int) else gate
        else:
            op = np.kron(op, np.eye(2)) if not isinstance(op, int) else np.eye(2)
    return op @ state

class QuantumCircuit:
    def __init__(self, n_qubits):
        self.n = n_qubits
        self.state = zero_state(n_qubits)
        
    # Apply Hadamard gate to a specific qubit
    def h(self, qubit_index):
        self.state = apply_gate(self.state, H, qubit_index, self.n)

    def measure(self):
            probs = np.abs(self.state.flatten())**2
            idx = np.random.choice(len(probs), p=probs)
            return format(idx, f'0{int(np.log2(len(probs)))}b')
