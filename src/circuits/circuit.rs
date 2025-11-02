use crate::circuits::QuantumCircuit;

use rand::rng;
use rand_distr::weighted::WeightedIndex;
use rand_distr::Distribution;
use num_complex::Complex64;

impl QuantumCircuit {
    /// Measure a specific qubit
    pub fn measure_qubit(&mut self, qubit_index: usize) -> char {
        let n = self.n;
        let dim = 1 << n;

        // Compute probability of qubit being 0 or 1
        let mut p0 = 0.0;
        let mut p1 = 0.0;

        for i in 0..dim {
            let bit = (i >> (n - 1 - qubit_index)) & 1;
            let amp = self.state[[i, 0]];
            if bit == 0 { p0 += amp.norm_sqr(); }
            else { p1 += amp.norm_sqr(); }
        }

        let dist = WeightedIndex::new(&[p0, p1]).unwrap();
        let mut rng = rng();
        let outcome = dist.sample(&mut rng);

        // Collapse state
        for i in 0..dim {
            let bit = (i >> (n - 1 - qubit_index)) & 1;
            if bit != outcome {
                self.state[[i, 0]] = Complex64::new(0.0, 0.0);
            }
        }

        // Normalize
        let norm: f64 = self.state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        for i in 0..dim {
            self.state[[i, 0]] /= norm;
        }

        if outcome == 0 { '0' } else { '1' }
    }

    /// Measure the quantum state and returns a bitstring
    pub fn measure(&self) -> String {
        let probs: Vec<f64> = self.state.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = rng();
        let idx = dist.sample(&mut rng);
        format!("{:0width$b}", idx, width = self.n)
    }
}
