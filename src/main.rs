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
use std::fs::create_dir;
use std::io::{Error, Write};


fn main() -> Result<(), Error> {
    // GA Constants
    let species = 10;
    let num_genes = species * species;
    let gapop = 30;
    let deme = 8;
    let pinfect = 0.2;
    let pmutate = 0.01;
    let evosteps = 5602;
    let save_every = gapop * 2;

    // initialize GA
    let mut mga = MGA::new(coexistence_search, gapop, num_genes, deme, pmutate, pinfect);

    // build file structure
    // file for tracking fitness in time
    let mut ffit = File::create("data/fitness3.csv")?;
    write!(ffit, "time,");
    for sp in 0..gapop {
        write!(ffit, "{},", sp);
    }
    write!(ffit, "\n");
    // directory for network structures
    create_dir("data/networks3")?;
    
    for i in 0..(evosteps / save_every) {
        // evole for some steps
        let fitness_history = mga.evolve(save_every as u32);
        
        // write "time" for the fitness file
        write!(ffit, "{},", (i*save_every) as i32);
        // network dir for this time step
        create_dir(format!("data/networks3/{}", i*save_every))?;
        
        // lots to do in this loop over genomes
        for j in 0..gapop {
            // first just write the fitness value
            write!(ffit, "{},", fitness_history[j]);
            // next make a file for the network structure and write to it
            let mut fnet = File::create(format!("data/networks3/{}/{}_adjmat_{}.csv", i*save_every, j, species))?;
            let adjmat = GLV::vec_to_mat(&mga.genomes[j], species);
            for spi in 0..species {
                for spj in 0..species {
                    write!(fnet, "{},", adjmat[spi][spj]);
                }
                write!(fnet, "\n");
            }

        }
        // this adds the newline we need at the end of the fitness line
        write!(ffit, "\n");
    }
    Ok(())
}

fn coexistence_search(genome: &Vec<u8>) -> f64 {
    // GLV Constants
    let species = 10;
    let coeffs = 10;
    let starts = 3;
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
    for i in 0..end_vec.len() {
        for j in 0..end_vec[0].len() {
            if end_vec[i][j] > 0.01 {
                s_avg += 1.0 / (total as f64);
            }
        }
    }
    let s_norm = s_avg / (species as f64);
    return s_norm
}

fn neutral_search(genome: &Vec<u8>) -> f64 {
    let fit: f64 = rand::thread_rng().gen();
    return fit
}
