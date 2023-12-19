use bot::{create_population, individual::Individual};

fn main() {
    let winning_position = 1175.0_f32;
    let population_size = 10;
    let mut population = create_population(population_size);
    let graded_retain_percent = 0.3;
    let nongraded_retain_percent = 0.2;
    let mut generation_count = 1;
    let mut successful: Vec<&Individual> = vec![];

    while successful.is_empty() {
        population
            .iter_mut()
            .for_each(|individual| individual.set_score(winning_position));

        successful = population
            .iter()
            .filter(|individual| individual.score == 1.0)
            .collect();

        generation_count += 1;
    }

    println!(
        "an individual figured out the correct force after {generation_count} generations: {:?}",
        successful
    );
}
