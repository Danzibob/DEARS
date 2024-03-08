pub trait Individual {
    fn mutate(&mut self);
    fn crossover(&mut self, other:&mut Self);
}

pub type Fitness<const N: usize> = Option<[f64; N]>;

