use dears::individual::*;
use dears::{mutation, crossover};

struct NumericIndividual {
    fitness: Fitness<1>,
    genes: Vec<usize>,
}

impl Individual for NumericIndividual {
    fn mutate(&mut self) {
        mutation::shuffle_indexes(&mut self.genes, 0.25)
    }

    fn crossover(&mut self, other: &mut Self) {
        crossover::one_point(&mut self.genes, &mut other.genes)
    }
}

fn create(genes: Vec<usize>) -> NumericIndividual {
    NumericIndividual{
        genes: genes, // Would usually have a random initializer here
        fitness: None
    }
}

fn main(){
    let mut ind1 = create(vec![1,2,3,4,5]);
    let mut ind2 = create(vec![9,8,7,6]);

    ind1.mutate();
    ind2.mutate();
    ind1.crossover(&mut ind2);

    println!("ind1: {:?}  ind2: {:?}", ind1.genes, ind2.genes);
}
