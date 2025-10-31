import numpy as np
from qucom.circuits import QuantumCircuit

def main():
    # 1. Create a 3-qubit circuit
    qc = QuantumCircuit(3)
    print("Initial state |000>:")
    print(qc.state)

    # 2. Apply Hadamard to qubit 0 (create superposition)
    qc.h(0)
    print("\nAfter applying H to qubit 0:")
    print(qc.state)

    # 3. Apply Hadamard to qubit 1
    qc.h(1)
    print("\nAfter applying H to qubit 1:")
    print(qc.state)

    # 4. Measure
    result = qc.measure()
    print("\nMeasurement result:", result)

    # 5. Run multiple measurements to see probabilities
    print("\nSampling measurement 10 times:")
    for _ in range(10):
        print(qc.measure())

if __name__ == "__main__":
    main()
