// internal modules
mod glv;
mod sbmga;
mod utils;
mod vmath;

use glv::GLV;
use utils::range;
use sbmga::MGA;

// external crates
use rand::Rng;
use std::fs::File;
use std::io::{Error, Write};


fn main() -> Result<(), Error> {
    // GA Constants
    let species = 5;
    let num_genes = species * species;
    let gapop = 5;
    let deme = 1;
    let pinfect = 0.3;
    let pmutate = 0.03;
    let evosteps = 10;

    // initialize GA
    let mut mga = MGA::new(coexistence_search, gapop, num_genes, deme, pmutate, pinfect);
    let fitness_history = mga.evolve(evosteps);
    
    //let mut mvec: Vec<u8> = Vec::new();
    //for _ in 0..num_genes {
    //    mvec.push(rand::thread_rng().gen_range(0..2));
    //}
    //let mat = GLV::vec_to_mat(&mvec, 5);
    //let mut glv = GLV::new(5);
    //glv.randomize_coeffs(&mat);
    //let emat = glv.a.clone();
    //let fitness_history = GLV::simulate(glv, vec![0.1, 0.2, 0.3, 0.8, 0.7], 50.0, 0.01);

    let mut fout = File::create("data/fitness.csv")?;
    write!(fout, "time,");
    for sp in 0..gapop {
        write!(fout, "{},", sp);
    }
    write!(fout, "\n");
    let times = range(0.0, 50.0, 0.01);
    for i in 0..fitness_history.len() {
        write!(fout, "{},", times[i]);
        for j in 0..gapop {
            write!(fout, "{},", fitness_history[i][j]);
        }
        write!(fout, "\n");
    }
    //for i in 0..species {
    //    println!("[{},{},{},{},{}]", emat[i][0], emat[i][1], emat[i][2], emat[i][3], emat[i][4]);
    //}

    Ok(())
}

fn coexistence_search(genome: &Vec<u8>) -> f64 {
    // GLV Constants
    let species = 5;
    let coeffs = 10;
    let starts = 10;
    let total = coeffs * starts;
    let simtime = 50.0;
    let simtimedt = 0.01;

    // set everything up
    let graph = GLV::vec_to_mat(genome, species);
    let mut end_vec: Vec<Vec<f64>> = Vec::new();
    
    // iterate over sets of coefficients
    for _c in 0..coeffs {
        let mut glv_sim = GLV::new(species);
        glv_sim.randomize_coeffs(&graph);

        let comp_mat = glv_sim.a;

        // for each set of coefficients we iterate over start conditions
        for _s in 0..starts {
            let mut glv_sim = GLV::new(species);
            glv_sim.a = comp_mat.clone();
            let mut init_state: Vec<f64> = vec![0.0; species];
            for i in 0..species {
                init_state[i] = rand::thread_rng().gen();
            }
            // run the simulation and save the final states
            let sim_run = GLV::simulate(glv_sim, init_state, simtime, simtimedt);
            let endstate = match sim_run.last().cloned() {
                Some(last) => last,
                _ => vec![0.0; species],
            };
            end_vec.push(endstate);
        }
    }
     
    // We will score each of these end states with species richness
    let mut s_avg = 0.0;
    for ev in end_vec {
        for spec in ev {
            if spec > 0.01 {
                s_avg += 1.0 / (total as f64);
            }
        }
    }
    let s_norm = s_avg / (species as f64);
    return s_norm
}
