pub enum DotResult {
    Scalar(f64),
    Vector(Vec<f64>),
}

pub enum ElemResult {
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}

pub trait VMath {
    fn dot(&self, x: &Vec<f64>) -> DotResult;
}

pub trait EMath {
    fn eadd(&self, x: &Vec<f64>) -> ElemResult;
    fn esub(&self, x: &Vec<f64>) -> ElemResult;
    fn ediv(&self, x: &Vec<f64>) -> ElemResult;
    fn emul(&self, x: &Vec<f64>) -> ElemResult;
}


impl VMath for Vec<f64> {
    fn dot(&self, x: &Vec<f64>) -> DotResult{
        assert_eq!(self.len(), x.len());
        let mut out = 0.0;
        for i in 0..self.len() {
            out += self[i] * x[i];
        }
        return DotResult::Scalar(out)
    }
}
impl EMath for Vec<f64> {
    fn eadd(&self, x: &Vec<f64>) -> ElemResult {
        assert_eq!(self.len(), x.len());
        let mut out = vec![0.0; x.len()];
        for i in 0..x.len() {
            out[i] = self[i] + x[i];
        }
        return ElemResult::Vector(out);
    }
    
    fn esub(&self, x: &Vec<f64>) -> ElemResult {
        assert_eq!(self.len(), x.len());
        let mut out = vec![0.0; x.len()];
        for i in 0..x.len() {
            out[i] = self[i] - x[i];
        }
        return ElemResult::Vector(out);
    }
    
    fn emul(&self, x: &Vec<f64>) -> ElemResult {
        assert_eq!(self.len(), x.len());
        let mut out = vec![0.0; x.len()];
        for i in 0..x.len() {
            out[i] = self[i] * x[i];
        }
        return ElemResult::Vector(out);
    }
    
    
    fn ediv(&self, x: &Vec<f64>) -> ElemResult {
        assert_eq!(self.len(), x.len());
        let mut out = vec![0.0; x.len()];
        for i in 0..x.len() {
            out[i] = self[i] / x[i];
        }
        return ElemResult::Vector(out);
    }
}

impl VMath for Vec<Vec<f64>> {
    fn dot(&self, x: &Vec<f64>) -> DotResult {
        assert_eq!(self[0].len(), x.len()); 
        let mut out = vec![0.0; x.len()];
        for i in 0..x.len() {
            out[i] = match self[i].dot(x) {
                DotResult::Scalar(o) => o,
                _ => -1.0,
                }
            }
        return DotResult::Vector(out)
    }
}

#[cfg(test)]
mod test_vmath {
    use super::*;
    #[test]
    fn test_dot_float() {
        let v1: Vec<f64> = vec![1.0, 2.0];
        let v2: Vec<f64> = vec![3.0, 4.0];

        let dp = match v1.dot(&v2) {
            DotResult::Scalar(prod) => prod,
            _ => -1.0
        };
        assert_eq!(dp, 11.0)
    }

    #[test]
    fn test_eadd_float() {
        let v1: Vec<f64> = vec![1.0, 2.0];
        let v2: Vec<f64> = vec![3.0, 3.0];

        let v3 = match v1.eadd(&v2) {
            ElemResult::Vector(o) => o,
            _ => vec![-1.0],
        };


        assert_eq!(v3, vec![4.0, 5.0])
    }

    #[test]
    fn test_esub_float() {
        let v1: Vec<f64> = vec![1.0, 2.0];
        let v2: Vec<f64> = vec![3.0, 3.0];

        let v3 = match v1.esub(&v2) {
            ElemResult::Vector(o) => o,
            _ => vec![-1.0],
        };

        assert_eq!(v3, vec![-2.0, -1.0])
    }

    #[test]
    fn test_ediv_float() {
        let v1: Vec<f64> = vec![1.0, 2.0];
        let v2: Vec<f64> = vec![2.0, 4.0];

        let v3 = match v1.ediv(&v2) {
            ElemResult::Vector(o) => o,
            _ => vec![-1.0],
        };

        assert_eq!(v3, vec![0.5, 0.5])
    }
    
    #[test]
    fn test_emul_float() {
        let v1: Vec<f64> = vec![1.0, 2.0];
        let v2: Vec<f64> = vec![2.0, 4.0];

        let v3 = match v1.emul(&v2) {
            ElemResult::Vector(o) => o,
            _ => vec![-1.0],
        };

        assert_eq!(v3, vec![2.0, 8.0])
    }
    

    #[test]
    fn test_dot_vec() {
        let m = vec![vec![1.0, 2.0],
                     vec![3.0, 4.0]];
        let v = vec![1.0, 2.0];
        let dp = match m.dot(&v) {
            DotResult::Vector(o) => o,
            _ => vec![-1.0]
        };
        assert_eq!(dp, vec![5.0, 11.0])
    }
    
    // #[test]
    // fn test_eadd_vec() {
        // let m1: Vec<f64> = vec![vec![1.0, 2.0],
                                // vec![3.0, 4.0]];
        // let m2: Vec<f64> = vec![vec![3.0, 3.0],
                                // vec![3.0, 3.0]];

        // let m3 = match m1.eadd(&m2) {
            // ElemResult::Matrix(o) => o,
            // _ => vec![vec![-1.0]],
        // };

        // let known = vec![vec![4.0, 5.0], 
                        //  vec![6.0, 7.0]];
        // assert_eq!(m3, known)
    // }
    
    // #[test]
    // fn test_esub_vec() {
        // let m1: Vec<f64> = vec![vec![1.0, 2.0],
                                // vec![3.0, 4.0]];
        // let m2: Vec<f64> = vec![vec![3.0, 3.0],
                                // vec![3.0, 3.0]];

        // let m3 = match m1.esub(&m2) {
            // ElemResult::Matrix(o) => o,
            // _ => vec![vec![-1.0]],
        // };

        // let known = vec![vec![-2.0, -1.0], 
                        //  vec![0.0, 1.0]];
        // assert_eq!(m3, known)
    // }
    
    // #[test]
    // fn test_emul_vec() {
        // let m1: Vec<f64> = vec![vec![1.0, 2.0],
                                // vec![3.0, 4.0]];
        // let m2: Vec<f64> = vec![vec![3.0, 3.0],
                                // vec![3.0, 3.0]];

        // let m3 = match m1.emul(&m2) {
            // ElemResult::Matrix(o) => o,
            // _ => vec![vec![-1.0]],
        // };

        // let known = vec![vec![3.0, 10.0], 
                        //  vec![9.0, 12.0]];
        // assert_eq!(m3, known)
    // }
    
    // #[test]
    // fn test_ediv_vec() {
        // let m1: Vec<f64> = vec![vec![1.0, 2.0],
                                // vec![3.0, 4.0]];
        // let m2: Vec<f64> = vec![vec![2.0, 2.0],
                                // vec![2.0, 2.0]];

        // let m3 = match m1.eadd(&m2) {
            // ElemResult::Matrix(o) => o,
            // _ => vec![vec![-1.0]],
        // };

        // let known = vec![vec![0.5, 1.0], 
                        //  vec![1.5, 2.0]];
        // assert_eq!(m3, known)
    // }
}