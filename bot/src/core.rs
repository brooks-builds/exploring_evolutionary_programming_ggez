use super::individual::Individual;
use rand::{seq::SliceRandom, thread_rng};

pub type Population = Vec<Individual>;

pub fn create_population(population_size: u8) -> Population {
    let mut population = vec![];

    for _ in 0..population_size {
        population.push(Individual::new());
    }
    population
}

pub fn selection(
    mut population: Population,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
) -> Population {
    population.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let keep_count = (population.len() as f32 * graded_retain_percent) as usize;
    let keep_ungraded_count = (population.len() as f32 * nongraded_retain_percent) as usize;
    let selected = population[0..keep_count].to_vec();
    let mut rng = thread_rng();

    let nongraded_selected = population[keep_count..]
        .choose_multiple(&mut rng, keep_ungraded_count)
        .map(ToOwned::to_owned)
        .collect();

    let selected = [selected, nongraded_selected].concat();

    selected
}

pub fn generation(
    population: Population,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    mutation_chance: f32,
) -> Population {
    let population_count = population.len();
    let mut survivors = selection(population, graded_retain_percent, nongraded_retain_percent);
    let mut children = vec![];
    let mut rng = thread_rng();

    while (children.len() + survivors.len()) < population_count {
        let parent_1 = survivors.choose(&mut rng).unwrap();
        let parent_2 = survivors.choose(&mut rng).unwrap();
        let child = Individual::new_from_parents(parent_1, parent_2, mutation_chance, &mut rng);

        children.push(child);
    }

    survivors.extend(children);

    survivors
}
