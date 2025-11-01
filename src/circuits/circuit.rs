use crate::circuits::QuantumCircuit;

use rand::rng;
use rand_distr::weighted::WeightedIndex;
use rand_distr::Distribution;

impl QuantumCircuit {
    /// Measure the quantum state and returns a bitstring
    pub fn measure(&self) -> String {
        let probs: Vec<f64> = self.state.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs).unwrap();
        let mut rng = rng();
        let idx = dist.sample(&mut rng);
        format!("{:0width$b}", idx, width = self.n)
    }
}
