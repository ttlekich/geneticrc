use geneticrs::{run, Opts};
use rand::Rng;
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
