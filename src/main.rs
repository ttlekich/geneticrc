use rand::seq::SliceRandom;
use rand::Rng;

use itertools::Itertools;

fn main() {
    let genotype = || {
        (0..1000)
            .map(|_| rand::thread_rng().gen_range(0..=1))
            .collect::<Vec<u32>>()
    };

    let fitness_function = |chromosome: &Vec<u32>| chromosome.iter().sum();

    let opts = Opts {
        population_size: 100,
    };

    let max_fitness = 1000;

    let best = run(&fitness_function, &genotype, max_fitness, &opts);

    println!("Final: {}", best);
}

pub struct Opts {
    population_size: u32,
}

fn initialize<T>(genotype: &dyn Fn() -> Vec<T>, opts: &Opts) -> Vec<Vec<T>> {
    let population_size = opts.population_size;

    (0..population_size).map(|_| genotype()).collect()
}

fn evaluate(
    mut population: Vec<Vec<u32>>,
    fitness_function: &dyn Fn(&Vec<u32>) -> u32,
    _opts: &Opts,
) -> Vec<Vec<u32>> {
    population.sort_by(|a, b| fitness_function(a).cmp(&fitness_function(b)));
    population
}

fn crossover(population: Vec<Vec<u32>>, _opts: &Opts) -> Vec<Vec<u32>>
// where
//     T: Clone,
{
    population
        .into_iter()
        .tuple_windows()
        .fold(vec![vec![]], |mut acc, (p1, p2)| {
            let crossover_point = rand::thread_rng().gen_range(0..p1.len());
            let (head_1, tail_1) = p1.split_at(crossover_point);
            let (head_2, tail_2) = p2.split_at(crossover_point);
            let mut child_1 = head_1.to_vec();
            let mut child_2 = head_2.to_vec();
            child_1.append(&mut tail_2.to_vec());
            child_2.append(&mut tail_1.to_vec());

            acc.push(child_1);
            acc.push(child_2);

            acc
        })
}

fn mutation(population: Vec<Vec<u32>>, _opts: &Opts) -> Vec<Vec<u32>>
// where
//     T: Clone,
{
    population
        .into_iter()
        .map(|mut chromosome| {
            if rand::thread_rng().gen::<f64>() < 0.05 {
                chromosome.shuffle(&mut rand::thread_rng())
            }
            chromosome
        })
        .collect()
}

fn evolve(mut population: Vec<Vec<u32>>, opts: &Opts) -> Vec<Vec<u32>>
// where
//     T: Clone,
{
    println!("Population Before: {}", population.len());
    population = crossover(population, opts);
    println!("Population Crossover: {}", population.len());
    population = mutation(population, opts);
    println!("Population Mutation: {}", population.len());
    population
}

fn run(
    fitness_function: &dyn Fn(&Vec<u32>) -> u32,
    genotype: &dyn Fn() -> Vec<u32>,
    max_fitness: u32,
    opts: &Opts,
) -> u32
// where
//     T: Clone,
{
    let mut population = initialize(genotype, opts);
    let mut best: u32 = 0;
    while best != max_fitness {
        population = evaluate(population, fitness_function, opts);
        best = fitness_function(population.first().unwrap());
        println!("Best: {}", best);
        population = evolve(population, opts);
        println!("Population: {}", population.len());
    }
    return best;
}
