use std::marker::PhantomData;

use num_traits::Float;
use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::{Normal, StandardNormal};

/// Trait defining an in-place mutation function to be implemented
/// by all mutation functions
pub trait Mutator<G: ?Sized> {
    fn mutate(&self, genome: &mut G);
}

/// Applies a per-element mutation sampled from a probability distribution
///
/// Modifies an individual (a slice of f64) in place, changing individual values with
/// probability `indpb` by a random amount from a distribution.
///
/// # Examples
///
/// ```
/// use dears::mutation::*;
/// use rand::distributions::Uniform;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// let dist = Uniform::new(0.3, 1.2);
/// // indpb = 0.5
/// let mutator = ByDist::new(dist, 0.5);
/// mutator.mutate(&mut vals);
/// // Vals has now been mutated!
/// println!("Uniform mutation: {:?}", vals);
/// ```
///
/// ```
/// use dears::mutation::*;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// // mu = 0.0, sigma = 1.0, indpb = 0.5
/// let mutator = ByDist::gaussian(
///     0.0,
///     1.0,
///     0.5
/// ).unwrap();
/// mutator.mutate(&mut vals);
/// // Vals has now been mutated!
/// println!("Gaussian: {:?}", vals);
/// ```
pub struct ByDist<F, D> {
    dist: D,
    // NOTE: this must be f64 because that's what Rng::gen_bool takes
    indpb: f64,
    _marker: PhantomData<F>,
}

impl<F: num_traits::Float, D: Distribution<F>> ByDist<F, D> {
    /// Creates a new `ByDist`
    pub fn new(dist: D, indpb: f64) -> Self {
        Self {
            dist,
            indpb,
            _marker: PhantomData,
        }
    }
}

impl<F> ByDist<F, rand_distr::Normal<F>>
where
    F: num_traits::Float,
    StandardNormal: Distribution<F>,
{
    pub fn gaussian(mean: F, stddev: F, indpb: f64) -> Result<Self, rand_distr::NormalError> {
        let dist = Normal::new(mean, stddev)?;
        Ok(Self {
            dist,
            indpb,
            _marker: PhantomData,
        })
    }
}

impl<F, D, G> Mutator<G> for ByDist<F, D>
where
    F: Float,
    D: Distribution<F>,
    G: AsMut<[F]>,
{
    fn mutate(&self, genome: &mut G) {
        let mut rng = rand::thread_rng();
        // Apply the random noise to selected genes
        for ind in genome.as_mut() {
            if rng.gen::<f64>() < self.indpb {
                let val = self.dist.sample(&mut rng);
                *ind = *ind + val;
            }
        }
    }
}

/// Swaps pairs of elements of any type, with probability `indpb` per item
///
/// Modifies an individual (a slice) in place, swapping individual values with
/// probability `indpb`. Can swap the same pair of elements multiple times.
///
/// # Examples
/// ```
/// use dears::mutation::*;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// let mutator = Shuffle { indpb: 0.4 };
/// mutator.mutate(&mut vals);
/// // Vals has now been mutated!
/// println!("Shuffled: {:?}", vals);
/// ```
pub struct Shuffle {
    pub indpb: f64,
}

impl<T: Clone> Mutator<[T]> for Shuffle {
    fn mutate(&self, genome: &mut [T]) {
        let mut rng = rand::thread_rng();
        let size = genome.len();
        // For each index of the list, if indpb is met
        // Swap with another random index of the list
        for idx in 0..size {
            if rng.gen::<f64>() < self.indpb {
                let mut swap_idx: usize = rng.gen_range(0..(size - 2));
                if swap_idx >= idx {
                    swap_idx += 1
                }
                genome.swap(idx, swap_idx);
            }
        }
    }
}

/// Flips random items in a slice of `bool`
///
/// Modifies an individual (a slice of bool) in place, flipping individual values with
/// probability `indpb`. 
///
/// # Examples
/// ```
/// use dears::mutation::*;
/// let mut vals = vec![false; 4];
/// let mutator = FlipBit { indpb: 0.5 };
/// mutator.mutate(&mut vals);
/// // Vals has now been mutated!
/// println!("Flipped:  {:?}", vals);
/// ```
pub struct FlipBit {
    pub indpb: f64,
}

impl Mutator<[bool]> for FlipBit {
    fn mutate(&self, genome: &mut [bool]) {
        let mut rng = rand::thread_rng();
        for i in 0..(genome.len()) {
            if rng.gen::<f64>() < self.indpb {
                genome[i] = !genome[i];
            }
        }
    }
}

// NB: These tests don't verify output, they just check the code compiles & runs
// Run the tests manually and view the output to ensure the values look consistent
#[cfg(test)]
mod tests {
    use crate::mutation::*;

    #[test]
    fn gaussian() {
        let mut test_input = vec![1.0, 2.0, 3.0, 4.0];
        let mutator = ByDist::gaussian(0.0, 1.0, 0.5).unwrap();
        mutator.mutate(&mut test_input);
        println!("Gaussian:  {:?}", test_input);
    }

    #[test]
    fn shuffle_indexes() {
        let mut test_input = vec![1.0, 2.0, 3.0, 4.0];
        let mutator = Shuffle { indpb: 0.4 };
        mutator.mutate(&mut test_input);
        println!("Shuffle:   {:?}", test_input);
    }

    #[test]
    fn flip_bit() {
        let mut test_input = vec![false; 4];
        let mutator = FlipBit { indpb: 0.4 };
        mutator.mutate(&mut test_input);
        println!("Flip Bit:  {:?}", test_input);
    }
}
