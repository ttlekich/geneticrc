use rand::Rng;
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
    T: Clone + ExactSizeIterator,
{
    let a = population
        .into_iter()
        .tuple_windows()
        .reduce(|acc, (p1, p2)| {
            let crossover_point = rand::thread_rng().gen_range(0..p1.len());
            (head_1, tail_1) = p1.group_by(|key| key < crossover_point);
            // (head_2, tail_2) = p2.split_off(crossover_point);
            // (child_1, child_2) = (head_1 + tail_2, head_2 + tail_1);
            // acc + child_1 + child_2
            acc
        });

    return;
}
