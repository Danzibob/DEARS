pub mod individual;
pub mod mutation;
pub mod crossover;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {

    }
}
