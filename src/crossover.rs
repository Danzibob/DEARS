use rand::Rng;

/// Performs one-point crossover between the two inputs
/// 
/// Modifies in place two individuals of the same type, swapping
/// their values after a random index. If the lengths don't match 
/// the values up to the length of the shorter individual are modified.
/// 
/// Individuals will always keep the same length after crossover.
/// 
/// # Examples
/// ```
/// use dears::crossover;
/// 
/// let mut ind1 = vec![1; 4];
/// let mut ind2 = vec![2; 7];
/// crossover::one_point(&mut ind1, &mut ind2);
/// println!("ind1 = {:?}, ind2 = {:?}", ind1, ind2);
/// // ind1 = [1, 1, 2, 2] ind2 = [2, 2, 1, 1, 2, 2, 2]
/// ```
pub fn one_point<T>(ind1: &mut [T], ind2: &mut [T]){
    let length = std::cmp::min(ind1.len(), ind2.len());
    assert!(length > 1, "Can't crossover individuals of length less than 2");
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(1..length);
    for i in crossover_point..length {
        std::mem::swap(&mut ind1[i], &mut ind2[i]);
    }
}