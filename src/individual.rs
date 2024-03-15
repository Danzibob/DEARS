use crate::crossover::*;
use crate::mutation::*;

pub type Fitness<const N: usize> = Option<[f64; N]>;

pub struct Population<G, M, C>
where
    M: Mutator<G>,
    C: Crossover<G>,
{
    individuals: Vec<G>,
    mutator: M,
    crossover: C,
}

// impl<T: Individual> Population<T> {
//     fn evaluate(&mut self, eval: fn(T) -> ???) {
//         for ind in self.individuals.iter_mut() {
//             ind.fitness = eval(ind);
//         }
//     }
// }
