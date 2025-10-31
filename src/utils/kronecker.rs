use ndarray::Array2;
use num_complex::Complex64;

pub fn kron(a: &Array2<Complex64>, b: &Array2<Complex64>) -> Array2<Complex64> {
    let (a_rows, a_cols) = a.dim();
    let (b_rows, b_cols) = b.dim();
    let mut result = Array2::<Complex64>::zeros((a_rows * b_rows, a_cols * b_cols));

    for i in 0..a_rows {
        for j in 0..a_cols {
            let a_ij = a[[i, j]];
            for k in 0..b_rows {
                for l in 0..b_cols {
                    result[[i*b_rows + k, j*b_cols + l]] = a_ij * b[[k,l]];
                }
            }
        }
    }
    result
}