use crate::vmath::VMath;
use crate::vmath::EMath;
use crate::vmath;
use crate::utils;
use crate::glv;
use rand_distr::Exp1;
use rand::prelude::*;

#[derive(Debug)]
pub struct GLV {
    pub n: usize,  // Number of species
    pub x: Vec<f64>,  // Density of each species
    pub r: Vec<f64>,  // Intrinsic growth rates
    pub a: Vec<Vec<f64>>,  // Competiton matrix
}

impl GLV {
    pub fn new(n_species: usize) -> GLV {
        let mut glv = GLV {
            n: n_species,
            x: vec![0.0; n_species],
            r: vec![1.0; n_species],
            a: vec![vec![0.0; n_species]; n_species],
        };
        // fill diagonal with ones
        for i in 0..n_species {
            glv.a[i][i] = 1.0;
        }

        return glv
    }

    pub fn step(&self, dt: f64) -> Vec<f64> {
        let rx = match self.r.emul(&self.x) {
            vmath::ElemResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        let interactions = match self.a.dot(&self.x) {
            vmath::DotResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        let one_minus = match vec![1.0; self.n].esub(&interactions) {
            vmath::ElemResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        let f = match rx.emul(&one_minus) {
            vmath::ElemResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        let update = match f.emul(&vec![dt; self.n]) {
            vmath::ElemResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        let new_vals = match self.x.eadd(&update) {
            vmath::ElemResult::Vector(o) => o,
            _ => vec![0.0; self.n]
        };
        return new_vals
    }

    fn randomize_coeffs(&mut self, template: Vec<Vec<u8>>) {
        assert_eq!(self.a.len(), template.len());
        assert_eq!(self.a[0].len(), template[0].len());
        for i in 0..self.a.len() {
            for j in 0..self.a[i].len() {
                // keeps things sort of grounded
                if i == j {
                    self.a[i][j] = 1.0;
                // heres our random coefficients along edges
                } else if template[i][j] == 1 {
                    self.a[i][j] = thread_rng().sample(Exp1);
                // otherwise its all zero
                } else {
                    self.a[i][j] = 0.0;
                }
            }
        }
    }

    fn simulate(mut model: glv::GLV, state: Vec<f64>, stop: f64, dt: f64) -> Vec<Vec<f64>> {
        let times = utils::range(0.0,stop, dt);
        let mut out_vec: Vec<Vec<f64>> = Vec::new();
        out_vec.push(state.clone());

        model.x = state;

        // update the state and get the state vector for this time step
        for _ in 1..times.len() {
            let new_x = model.step(dt);
            out_vec.push(new_x.clone());
            model.x = new_x;
        }

        return out_vec
    }
 
    fn vec_to_mat<T: Copy>(v: &Vec<T>, size: usize) -> Vec<Vec<T>> {
        let vv = v.clone();
        let mut m: Vec<Vec<T>> = Vec::with_capacity(size);
        let mut new_row: Vec<T> = Vec::with_capacity(size);
        
        for i in 0..v.len() {
            new_row.push(vv[i]);
            if (i > 0) & (i % size == 2) {
                m.push(new_row.clone());
                new_row.clear()
            }
        }
        return m
    }
}

#[cfg(test)]
mod test_glv {
    use super::GLV;

    #[test]
    fn test_new() {
        let glv = GLV::new(3);
        let glv_a = glv.a;
        let glv_r = glv.r;
        
        assert_eq!(glv_a[0][0], 1.0);
        assert_eq!(glv_a[0][1], 0.0);
        assert_eq!(glv_a[1][1], 1.0);
        assert_eq!(glv_r[0], 1.0)
    }

    #[test]
    fn test_step() {
        let mut glv = GLV::new(3);
        // set intransitive cycle for no real reason
        glv.a[0][1] = 0.5;
        glv.a[1][2] = 0.5;
        glv.a[2][0] = 0.5;
        // init populations
        glv.x = vec![0.5, 0.4, 0.6];

        let new_x = glv.step(0.01);
        assert_eq!(new_x[0], 0.5015);
        assert_eq!(new_x[1], 0.4012);
        assert_eq!(new_x[2], 0.6009);
    }

    #[test]
    fn test_randomize_coeffs() {
        let mut glv = GLV::new(3);
        let graph = vec![vec![0, 1, 0],
                         vec![1, 0, 1],
                         vec![1, 0, 0]];
        
        glv.randomize_coeffs(graph);

        assert!(glv.a[0][1] > 0.0);
        assert_eq!(glv.a[1][1], 1.0);
        assert_eq!(glv.a[2][1], 0.0)
    }

    #[test]
    fn test_simulate() {
        let mut glv = GLV::new(3);
        // set intransitive cycle for no real reason
        glv.a[0][1] = 0.5;
        glv.a[1][2] = 0.5;
        glv.a[2][0] = 0.5;

        // init populations
        glv.x = vec![0.5, 0.4, 0.6];

        let out = GLV::simulate(glv, vec![0.5, 0.45, 0.55], 1.0, 0.001);

        assert!((out[99][0] > 0.0) & (out[99][0] < 1.0));
        assert!((out[99][1] > 0.0) & (out[99][1] < 1.0));
        assert!((out[99][2] > 0.0) & (out[99][2] < 1.0))
    }

    #[test]
    fn test_vec_to_mat() {
        let in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let out_vec = vec![vec![1, 2, 3],
                           vec![4, 5, 6],
                           vec![7, 8, 9]];
        let test_out = GLV::vec_to_mat(&in_vec, 3);
        assert_eq!(out_vec[0][1], test_out[0][1]);
        assert_eq!(out_vec[1][0], test_out[1][0]);
        assert_eq!(out_vec[2][2], test_out[2][2]);
    }
}