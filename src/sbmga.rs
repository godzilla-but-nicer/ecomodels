use rand::Rng;

pub struct MGA {
    pub fitness: fn(&Vec<u8>) -> f64,
    pub fitness_values: Vec<f64>,
    pub pop_size: usize,
    pub gene_size: usize,
    deme_size: usize,
    pub genomes: Vec<Vec<u8>>,
    mut_prob: f64,
    inf_prob: f64,
}

impl MGA {
    pub fn new(ffunc: fn(&Vec<u8>) -> f64, population: usize, genes: usize, 
           deme: usize, mp: f64, ip: f64) -> MGA {
        // this can break shit
        assert!(population - 1 >= deme);

        // rng for initialization
        let mut rng = rand::thread_rng();
        let mut fvec: Vec<f64> = Vec::new();
        let mut gvec: Vec<Vec<u8>> = Vec::new();

        // make a random vector of zeros and ones
        for _i in 0..population {
            let mut new_genome: Vec<u8> = Vec::new();
            for _j in 0..genes {
                new_genome.push(rng.gen_range(0..2));
            }
            gvec.push(new_genome);
            fvec.push(0.0);
        }

        let out_mga = MGA {
            fitness: ffunc,
            fitness_values: fvec,
            pop_size: population,
            gene_size: genes,
            deme_size: deme,
            genomes: gvec,
            mut_prob: mp,
            inf_prob: ip,
        };
        return out_mga
    }

    fn pick_competitors(&self, i: usize, fixed_i: bool) -> [usize; 2] {
        if fixed_i {
            let j = rand::thread_rng().gen_range((i + 1)..(i + self.deme_size + 1)) % self.pop_size;
            return [i, j];
        } else {
            let _i = rand::thread_rng().gen_range(0..self.pop_size);
            let j = rand::thread_rng().gen_range((_i + 1)..(_i + self.deme_size + 1)) % self.pop_size;
            return [_i, j];
        }

    }

    fn compete(&self, i: usize, j: usize) -> [usize; 2] {
        // calculate fitness values
        let fit_i = (self.fitness)(&self.genomes[i]);
        let fit_j = (self.fitness)(&self.genomes[j]);

        // init win lose idx
        let win: usize;
        let lose: usize;

        // determine tournament outcome
        if fit_i > fit_j {
            win = i;
            lose = j;
        } else {
            win = j;
            lose = i;
        }
        return [win, lose]
    }

    fn step(&mut self) {
        // pick competitors and get the winners
        let comps = self.pick_competitors(0, false);
        let outs = self.compete(comps[0], comps[1]);
        println!("{} beats {}", outs[0], outs[1]);

        for gene_i in 0..self.genomes[0].len() {
            // infect
            let inf_roll: f64 = rand::thread_rng().gen();
            if inf_roll < self.inf_prob {
                self.genomes[outs[1]][gene_i] = self.genomes[outs[0]][gene_i];
            }

            // mutate
            let mut_roll: f64 = rand::thread_rng().gen();
            if mut_roll < self.mut_prob {
                self.genomes[outs[1]][gene_i] = (self.genomes[outs[1]][gene_i] + 1) % 2;
            }
        }
    }

    fn get_fitness(&self) -> Vec<f64> {
        let mut fit_vec: Vec<f64> = Vec::new();
        for i in 0..self.genomes.len() {
            fit_vec.push((self.fitness)(&self.genomes[i]));
        }
        return fit_vec
    }

    pub fn evolve(&mut self, n_steps: u32) -> Vec<Vec<f64>> {
        let mut fit_record: Vec<Vec<f64>> = Vec::new();

        for _ in 0..n_steps {
            self.step();
            fit_record.push(self.get_fitness());
            println!("Step Complete");
        }

        return fit_record
    }
}

#[cfg(test)]
mod test_mga {
    use super::*;

    fn ffunc_test(genes: &Vec<u8>) -> f64 {
        let mut gene_sum = 0;
        for i in 0..genes.len() {
            gene_sum += genes[i];
        }
        return gene_sum as f64
    }
    #[test]
    fn test_new() {
        let mga = MGA::new(ffunc_test, 3, 3, 2, 0.01, 0.01);
        assert_eq!(mga.genomes.len(), 3);
        assert_eq!(mga.genomes[0].len(), 3);
        assert!((mga.fitness)(&vec![1, 1, 1]) == 3.0)
    }

    #[test]
    fn test_pick_competitors() {
        let mga = MGA::new(ffunc_test, 3, 3, 2, 0.01, 0.01);
        let comp = mga.pick_competitors(0, false);
        assert!(comp[0] < mga.pop_size);
        assert!(comp[1] < mga.pop_size);
        assert!(comp[0] != comp[1])
    }

    #[test]
    fn test_compete() {
        // initialize and set matrix to known solution
        let mut mga = MGA::new(ffunc_test, 2, 3, 1, 0.01, 0.01);
        mga.genomes = vec![vec![0, 1, 1],
                           vec![1, 1, 1]];

        // get competitors
        let outcomes = mga.compete(0, 1);

        assert_eq!(outcomes[0], 1);
        assert_eq!(outcomes[1], 0)
    }
    #[test]
    fn test_step() {
        // initialize and set matrix to known solution
        let mut mga = MGA::new(ffunc_test, 2, 3, 1, 0.0, 1.0);
        mga.genomes = vec![vec![0, 1, 1],
                           vec![1, 1, 1]];

        // get competitors
        mga.step();

        assert_eq!((mga.fitness)(&mga.genomes[0]), 3.0);
        assert_eq!((mga.fitness)(&mga.genomes[1]), 3.0)
    }
    #[test]
    fn test_evolve() {
        // initialize and set matrix to known solution
        let mut mga = MGA::new(ffunc_test, 3, 3, 2, 0.0, 1.0);
        mga.genomes = vec![vec![0, 0, 1],
                           vec![1, 1, 1],
                           vec![0, 0, 0]];

        // get competitors
        let fit_history = mga.evolve(30);
        let last_i = fit_history.len() - 1;
        let mut sum: f64 = 0.0;
        for j in 0..fit_history[last_i].len() {
            sum += fit_history[last_i][j] / 3.0;
        }

        assert_eq!(sum, 3.0)
    }
}