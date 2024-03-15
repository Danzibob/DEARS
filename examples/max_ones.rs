use dears::mutation::*;
use rand::{thread_rng, Rng};

type Genome = [bool; 10];
type Fitness = [f64; 1];

fn fitness(individual: Genome) -> Fitness {
    return [
        // fitness is the count of "True" in the array
        individual.iter().filter(|&x| *x).count() as f64
        ]
}

fn main(){
    const POP_SIZE: usize = 100;
    const N_GENS: usize = 20;
    const MUTATE_PROB: f64 = 0.1;

    let mut rng = thread_rng();

    let mut pop:Vec<Genome> = Vec::new();
    for _ in 0..POP_SIZE {
        pop.push([false; 10])
    }

    let mutator = FlipBit { indpb: 0.4 };

    for gen in 0..N_GENS {
        for ind in 0..POP_SIZE {
            if rng.gen::<f64>() < MUTATE_PROB {
                mutator.mutate(&mut pop[ind]);
            }
        }
        println!("Completed gen {}", gen+1);
    }

    for ind in 0..POP_SIZE {
        println!("{:?}", fitness(pop[ind]));
    }
}
