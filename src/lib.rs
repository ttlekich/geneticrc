use rand::seq::SliceRandom;
use rand::Rng;

use itertools::Itertools;
pub struct Opts {
    pub population_size: u32,
}

fn initialize<T>(genotype: &dyn Fn() -> Vec<T>, opts: &Opts) -> Vec<Vec<T>> {
    let population_size = opts.population_size;

    (0..population_size).map(|_| genotype()).collect()
}

fn evaluate<T>(
    mut population: Vec<Vec<T>>,
    fitness_function: &dyn Fn(&Vec<T>) -> u32,
    _opts: &Opts,
) -> Vec<Vec<T>> {
    population.sort_by(|a, b| fitness_function(b).cmp(&fitness_function(a)));
    population
}

fn crossover<T>(population: Vec<Vec<T>>, _opts: &Opts) -> Vec<Vec<T>>
where
    T: Clone,
{
    population
        .into_iter()
        .tuples()
        .fold(vec![], |mut acc, (p1, p2)| {
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

fn mutation<T>(population: Vec<Vec<T>>, _opts: &Opts) -> Vec<Vec<T>>
where
    T: Clone,
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

fn evolve<T>(mut population: Vec<Vec<T>>, opts: &Opts) -> Vec<Vec<T>>
where
    T: Clone,
{
    population = crossover(population, opts);
    population = mutation(population, opts);
    population
}

pub fn run<T>(
    fitness_function: &dyn Fn(&Vec<T>) -> u32,
    genotype: &dyn Fn() -> Vec<T>,
    max_fitness: u32,
    opts: &Opts,
) -> u32
where
    T: Clone,
{
    let mut population = initialize(genotype, opts);
    let mut best: u32 = 0;
    while best != max_fitness {
        println!("Best: {}", best);
        population = evaluate(population, fitness_function, opts);
        best = fitness_function(population.first().unwrap());
        population = evolve(population, opts);
    }
    return best;
}
