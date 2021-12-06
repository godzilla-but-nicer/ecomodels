pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let length_float: f64 = (stop - start).floor() / step;
    let length: usize = length_float as usize;

    let mut out_vec = vec![start; length];

    // fill the actual thing
    for i in 1..out_vec.len() {
        out_vec[i] = out_vec[i-1] + step;
    }
    out_vec
}