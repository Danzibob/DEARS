use std::usize;

use rand::Rng;

use crate::population::*;

/// Trait defining a selection function that uses fitnesses in a population
pub trait SelectOne<F> {
    fn select(&self, fitnesses: &Vec<F>) -> usize;
}

pub trait SelectMany<F> {
    fn select_n(&self, fitnesses: &Vec<F>, n: usize) -> Vec<usize>;
}

impl<F> SelectMany<F> for dyn SelectOne<F> {
    fn select_n(&self, fitnesses: &Vec<F>, n: usize) -> Vec<usize> {
        let mut selected = Vec::with_capacity(n);
        for _ in 0..n {
            selected.push(self.select(fitnesses));
        }
        selected
    }
}



pub struct TournamentSelection {
    pub tournament_size: usize,
}

impl<F: PartialOrd + Copy> SelectOne<F> for TournamentSelection {
    fn select(&self, fitnesses: &Vec<F>) -> usize {
        let len = fitnesses.len();
        assert!(len > 0, "Can't select from empty fitnesses vector");

        let mut rng = rand::thread_rng();
        let options = (0..self.tournament_size).map(|_| rng.gen_range(0..len));
        options.max_by(|&a, &b| {
            fitnesses[a].partial_cmp(&fitnesses[b])
                        .expect("Failed to compare fitnesses, are they NaN?")
        }).expect("Tournament size can't be 0")
    }
}

pub struct SelBest {}

// impl<const N: usize, T: PartialOrd> Selector<[T; N]> for TournamentSelection {
//     fn select(&self, fitnesses: Vec<[f64; N]>) -> usize {
//     }
// }