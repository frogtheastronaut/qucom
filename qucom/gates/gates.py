import numpy as np

# Hadamard Gate
HADAMARD = (1/2**0.5) * np.array([[1, 1],
								  [1, -1]], dtype=complex)

PAULI_X = np.array([[0, 1],
                   [1, 0]], dtype=complex)