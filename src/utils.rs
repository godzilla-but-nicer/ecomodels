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

pub fn hamming<T: std::cmp::Eq>(x: &Vec<T>, y: &Vec<T>) -> usize {
    let mut dist: usize = 0;
    for i in 0..x.len() {
        if x[i] != y[i] {
            dist += 1;
        }
    }
    return dist
}

#[cfg(test)]
mod test_utils {
    use crate::utils::hamming;
    #[test]
    fn test_hamming() {
        let x1 = vec![2, 3, 0];
        let x2 = vec![2, 4, 0];

        assert_eq!(hamming(&x1, &x2), 1)
    }
}