use rand::Rng;
use rand_distr::{Distribution, Normal};

/// Applies a per-element gaussian mutation of mean `mu` and std dev `sigma`
///
/// Modifies an individual (a slice of f64) in place, changing individual values with
/// probability `indpb` by a random amount from a gaussian distribution defined by
/// the mean `mu` and standard deviation `sigma`.
///
/// # Examples
/// ```
/// use dears::mutation;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// // mu = 0.0, sigma = 1.0, indpb = 0.5
/// mutation::gaussian(&mut vals, 0.0, 1.0, 0.5);
/// // Vals has now been mutated!
/// println!("Gaussian: {:?}", vals);
/// ```
pub fn gaussian(individual: &mut [f64], mu: f64, sigma: f64, indpb: f64) {
    let size = individual.len();
    let mus = vec![mu; size];
    let sigmas = vec![sigma; size];
    gaussian_list(individual, &mus, &sigmas, indpb);
}

/// Applies a per-element gaussian mutation using means `mus` and std devs `sigmas`
///
/// Modifies an individual (a slice of f64) in place, changing individual values with
/// probability `indpb` by a random amount from a gaussian distribution defined by
/// the associated mean and standard deviation from `mus` and `sigmas`
/// 
/// If you want the same distribution (same mu and sigma) for all items, use [`gaussian`]
/// 
/// # Panics
/// 
/// The 
///
/// # Examples
/// ```
/// use dears::mutation;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// let mus = vec![0.0; 4];
/// let sigmas = vec![0.1, 1.0, 10.0, 100.0];
/// mutation::gaussian_list(&mut vals, &mus, &sigmas, 0.5);
/// // Vals has now been mutated!
/// println!("Gaussian: {:?}", vals);
/// ```
/// 
/// [`gaussian`]: ./fn.gaussian.html
pub fn gaussian_list(individual: &mut [f64], mus: &[f64], sigmas: &[f64], indpb: f64) {
    // Panic with error message if lengths of inputs don't match
    assert_eq!(individual.len(), mus.len(), "Length of `mus` must match length of individual");
    assert_eq!(individual.len(), sigmas.len(), "Length of `sigmas` must match length of individual");
    let mut rng = rand::thread_rng();
    let params = mus.iter().zip(sigmas.iter());
    for (ind, (&mu, &sigma)) in individual.iter_mut().zip(params) {
        if rng.gen::<f64>() < indpb {
            let normal = Normal::new(mu, sigma).unwrap();
            let val = normal.sample(&mut rng);
            *ind += val;
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
/// use dears::mutation;
/// let mut vals = vec![1.0, 2.0, 3.0, 4.0];
/// mutation::shuffle_indexes(&mut vals, 0.5);
/// // Vals has now been mutated!
/// println!("Shuffled: {:?}", vals);
/// ```
pub fn shuffle_indexes<T: Copy>(individual: &mut [T], indpb: f64) {
    let mut rng = rand::thread_rng();
    let size = individual.len();
    for idx in 0..size {
        if rng.gen::<f64>() < indpb {
            let mut swap_idx: usize = rng.gen_range(0..(size - 2));
            if swap_idx >= idx {
                swap_idx += 1
            }
            individual.swap(idx, swap_idx);
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
/// use dears::mutation;
/// let mut vals = vec![false; 4];
/// mutation::flip_bit(&mut vals, 0.5);
/// // Vals has now been mutated!
/// println!("Flipped:  {:?}", vals);
pub fn flip_bit(individual: &mut [bool], indpb: f64) {
    let mut rng = rand::thread_rng();
    for i in 0..(individual.len()) {
        if rng.gen::<f64>() < indpb {
            individual[i] = !individual[i];
        }
    }
}


// NB: These tests don't verify output, they just check the code compiles & runs
// Run the tests manually and view the output to ensure the values look consistent
#[cfg(test)]
mod tests {
    #[test]
    fn gaussian() {
        let mut test_input = vec![1.0, 2.0, 3.0, 4.0];
        let mu = 0.0;
        let sigma = 1.0;
        let indpb = 0.6;
        super::gaussian(&mut test_input, mu, sigma, indpb);
        println!("Gaussian:  {:?}", test_input);
    }

    #[test]
    #[should_panic(expected = "Length of `sigmas` must match length of individual")]
    fn gaussian_list() {
        let mut test_input = vec![1.0, 2.0, 3.0, 4.0];
        let mus = vec![0.0; 4];
        let sigmas = vec![1.0; 3]; // Incorrect length
        super::gaussian_list(&mut test_input, &mus, &sigmas, 0.5);
        // This should panic!
    }

    #[test]
    fn shuffle_indexes() {
        let mut test_input = vec![1.0, 2.0, 3.0, 4.0];
        let indpb = 0.6;
        super::shuffle_indexes(&mut test_input, indpb);
        println!("Shuffle:   {:?}", test_input);
    }

    #[test]
    fn flip_bit() {
        let mut test_input = vec![false; 4];
        let indpb = 0.6;
        super::flip_bit(&mut test_input, indpb);
        println!("Flip Bit:  {:?}", test_input);
    }
}
