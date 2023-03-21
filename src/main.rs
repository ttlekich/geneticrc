use std::slice::{Iter, Windows};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

pub struct Opts {
    population_size: u32,
}

fn initialize<T>(genotype: &dyn Fn() -> T, opts: Opts) {
    let population_size = opts.population_size;

    for _ in 0..population_size {
        genotype();
    }
}

fn evaluate<T>(mut population: Vec<T>, fitness_function: &dyn Fn(&T) -> u32, opts: Opts) {
    population.sort_by(|a, b| fitness_function(a).cmp(&fitness_function(b)))
}

fn crossover<T>(mut population: Vec<T>, opts: Opts)
where
    T: Clone,
{
    let new_population = population.into_iter().tuple_windows::<(T, T)>();

    new_population.reduce(|a, b| {
        let (a, b) = (a.0, b.1);
        let (a, b) = (a.clone(), b.clone());
        (a, b)
    });
}
