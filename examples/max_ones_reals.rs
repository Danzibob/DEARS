use dears::individual::*;

// creator.create("FitnessMax", base.Fitness, weights=(1.0,))
// creator.create("Individual", list, fitness=creator.FitnessMax)

// toolbox.register("attr_real", random.random)
// toolbox.register("individual", tools.initRepeat, creator.Individual, toolbox.attr_real, n=10)

const INDIVIDUAL_SIZE: usize = 10;
struct OnesIndividual {
    fitness: Fitness<1>,
    genes: Vec<f64>
}



fn main() {

}