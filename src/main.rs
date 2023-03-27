use rand::Rng;
use std::slice::{Iter, Windows};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

pub struct Opts {
    population_size: u32,
}

fn initialize<T>(genotype: &dyn Fn() -> Vec<T>, opts: Opts) {
    let population_size = opts.population_size;

    for _ in 0..population_size {
        genotype();
    }
}

fn evaluate<T>(mut population: Vec<Vec<T>>, fitness_function: &dyn Fn(&Vec<T>) -> u32, opts: Opts) {
    population.sort_by(|a, b| fitness_function(a).cmp(&fitness_function(b)))
}

fn crossover<T>(mut population: Vec<Vec<T>>, opts: Opts) -> Vec<Vec<T>>
where
    T: Clone + ExactSizeIterator,
{
    population
        .into_iter()
        .tuple_windows()
        .fold(vec![vec![]], |acc, (p1, p2)| {
            let crossover_point = rand::thread_rng().gen_range(0..p1.len());
            let (head_1, tail_1) = p1.split_at(crossover_point);
            let (head_2, tail_2) = p2.split_at(crossover_point);
            let mut child1 = head_1.to_vec();
            let mut child2 = head_2.to_vec();
            child1.append(&mut tail_2.to_vec());
            child2.append(&mut tail_1.to_vec());

            acc
        })
}
