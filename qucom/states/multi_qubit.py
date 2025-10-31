import numpy as np
from .states import ZERO, ONE

def basis_state(bitstring):
    state = np.array([[1]], dtype=complex)
    for bit in bitstring:
        state = np.kron(state, ZERO if bit=='0' else ONE)
    return state

def superposition(bitstrings):
    psi = sum(basis_state(b) for b in bitstrings)
    return psi / np.linalg.norm(psi)
