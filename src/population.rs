use rand::thread_rng;
use rayon::prelude::*;
use crate::crossover::*;
use crate::mutation::*;
use crate::selection::*;

pub type Fitness = [f64; 3];

pub struct Population<G, M, C, S, F>
where
    M: Mutator<G>,
    C: Crossover<G>,
    S: SelectMany<F>,
    F: Clone
{
    individuals: Vec<G>,
    fitnesses: Vec<F>,
    mutator: M,
    crossover: C,
    selector: S
}

impl<G, M, C, S, F> Population<G, M, C, S, F>
where
    M: Mutator<G>,
    C: Crossover<G>,
    S: SelectMany<F>,
    F: Clone
{
    fn mutate_with_chance(&mut self, indpb: f64) {
        self.individuals.iter_mut().for_each(|x| {
            if rand::random::<f64>() > indpb {
                self.mutator.mutate(x);
            }
        });
    }

    fn select(&self, n: usize) -> Vec<usize>{
        todo!()
    }
}

// impl<T: Individual> Population<T> {
//     fn evaluate(&mut self, eval: fn(T) -> ???) {
//         for ind in self.individuals.iter_mut() {
//             ind.fitness = eval(ind);
//         }
//     }
// }
