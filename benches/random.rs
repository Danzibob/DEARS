#![feature(test)]

extern crate test;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use test::Bencher;

pub trait Mutator<G: ?Sized> {
    fn mutate(&self, genome: &mut G);
}

pub struct Gaussian {
    pub mu: f64,
    pub sigma: f64,
    pub indpb: f64,
}

impl Mutator<[f64]> for Gaussian {
    fn mutate(&self, genome: &mut [f64]) {
        // Initialize the random distribution
        let mut rng = rand::thread_rng();
        let normal = Normal::new(self.mu, self.sigma).unwrap_or_else(|_| {
            panic!(
                "Invalid args to Normal Distribution: sigma={} mu={}",
                self.sigma, self.mu
            )
        });
        // Apply the random noise to selected genes
        for ind in genome.iter_mut() {
            if rng.gen_bool(self.indpb) {
                let val = normal.sample(&mut rng);
                *ind += val;
            }
        }
    }
}

// ---------------

pub trait RngMutator<G: ?Sized> {
    fn mutate(&self, genome: &mut G, rng: &mut ThreadRng, dist: Normal<f64>);
}

pub struct RngGaussian {
    pub mu: f64,
    pub sigma: f64,
    pub indpb: f64,
}

impl RngMutator<[f64]> for RngGaussian {
    fn mutate(&self, genome: &mut [f64], rng: &mut ThreadRng, dist: Normal<f64>) {
        // Apply the random noise to selected genes
        for ind in genome.iter_mut() {
            if rng.gen_bool(self.indpb) {
                let val = dist.sample(rng);
                *ind += val;
            }
        }
    }
}

#[bench]
fn outer_random(b: &mut Bencher) {
    b.iter(|| {
        let mut pop = [[0.0; 10]; 1000];
        let mutator = RngGaussian {
            mu: 0.0,
            sigma: 1.0,
            indpb: 0.5,
        };
        let mut rng = thread_rng();
        let normal = Normal::new(mutator.mu, mutator.sigma).unwrap_or_else(|_| {
            panic!(
                "Invalid args to Normal Distribution: sigma={} mu={}",
                mutator.sigma, mutator.mu
            )
        });

        for _ in 0..1000 {
            pop.iter_mut()
                .for_each(|x| mutator.mutate(x, &mut rng, normal));
        }
    });
}

#[bench]
fn inner_random(b: &mut Bencher) {
    b.iter(|| {
        let mut pop = [[0.0; 10]; 1000];
        let mutator = Gaussian {
            mu: 0.0,
            sigma: 1.0,
            indpb: 0.5,
        };

        for _ in 0..1000 {
            pop.iter_mut().for_each(|x| mutator.mutate(x));
        }
    });
}

#[bench]
fn parallel_random(b: &mut Bencher) {
    b.iter(|| {
        let mut pop = [[0.0; 10]; 1000];
        let mutator = Gaussian {
            mu: 0.0,
            sigma: 1.0,
            indpb: 0.5,
        };

        for _ in 0..1000 {
            pop.par_iter_mut().for_each(|x| mutator.mutate(x));
        }
    });
}
