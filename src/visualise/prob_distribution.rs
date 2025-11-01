use ndarray::Array2;
use num_complex::Complex64;
use plotters::prelude::*;
use std::collections::HashMap;

/// Compute exact probabilities for each basis state
pub fn state_probabilities(state: &Array2<Complex64>) -> HashMap<String, f64> {
    let n_qubits = (state.len() as f64).log2() as usize;
    let mut probs = HashMap::new();

    for (i, amp) in state.iter().enumerate() {
        let bitstring = format!("{:0width$b}", i, width = n_qubits);
        probs.insert(bitstring, amp.norm_sqr());
    }

    probs
}

/// Plot probabilities as a bar chart
pub fn plot_probabilities(
    probs: &HashMap<String, f64>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Collect keys in a consistent sorted order
    let mut states: Vec<String> = probs.keys().cloned().collect();
    states.sort();

    let max_prob = probs.values().cloned().fold(0.0 / 0.0, f64::max);

    // Define drawing area
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = 0f64..states.len() as f64;
    let y_range = 0.0..(max_prob * 1.2);

    let mut chart = ChartBuilder::on(&root)
        .caption("Quantum State Probabilities", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    chart
        .configure_mesh()
        .x_labels(states.len())
        .x_label_formatter(&|idx| {
            let idx = *idx as usize;
            if idx < states.len() {
                states[idx].clone()
            } else {
                "".to_string()
            }
        })
        .y_desc("Probability")
        .x_desc("State")
        .draw()?;

    // Draw bars
    chart.draw_series(states.iter().enumerate().map(|(idx, state)| {
        let p = probs[state];
        let bar_width = 0.8; // fit inside the chart
        Rectangle::new(
            [(idx as f64, 0.0), (idx as f64 + bar_width, p)],
            BLUE.filled(),
        )
    }))?;

    Ok(())
}
