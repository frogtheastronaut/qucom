use ndarray::Array2;
use num_complex::Complex64;
use plotters::prelude::*;
use std::collections::HashMap;

/// compute state probabilities from state vector
pub fn state_probabilities(state: &Array2<Complex64>) -> HashMap<String, f64> {
    let n_qubits = (state.len() as f64).log2() as usize;
    let mut probs = HashMap::new();
    for (i, amp) in state.iter().enumerate() {
        let bitstring = format!("{:0width$b}", i, width = n_qubits);
        probs.insert(bitstring, amp.norm_sqr());
    }
    probs
}

/// plot probabilities as a bar chart
pub fn plot_probabilities(
    probs: &HashMap<String, f64>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut states: Vec<String> = probs.keys().cloned().collect();
    states.sort();

    let max_prob = probs.values().cloned().fold(0.0, f64::max);

    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Probabilities", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(-0.5..(states.len() as f64 - 0.5), 0.0..(max_prob * 1.2))?;

    
    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(states.len())
        .x_label_formatter(&|idx| {
            let i = *idx as usize; // cast f64 -> usize safely
            if i < states.len() {
                states[i].clone()
            } else {
                "".to_string()
            }
        })
        .y_desc("Probability")
        .x_desc("State")
        .draw()?;

    
    for (idx, state) in states.iter().enumerate() {
        let p = probs[state];
        let bar_width = 0.8;

        
        chart.draw_series(std::iter::once(Rectangle::new(
            [(idx as f64 - bar_width / 2.0, 0.0), (idx as f64 + bar_width / 2.0, p)],
            BLUE.filled(),
        )))?;

        
        chart.draw_series(std::iter::once(Text::new(
            format!("{:.2}", p),
            (idx as f64, p + 0.01),
            ("sans-serif", 15.0).into_font(),
        )))?;
    }

    Ok(())
}
