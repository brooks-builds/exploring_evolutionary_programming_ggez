use eyre::{bail, Result};
use individual::Individual;
use rand::{
    distributions::{Alphanumeric, DistString, Uniform},
    seq::SliceRandom,
    thread_rng, Rng,
};
pub mod individual;

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
    target_x: f32,
) -> Vec<String> {
    population.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let keep_count = (chromosomes.len() as f32 * graded_retain_percent) as usize;
    let keep_ungraded_count = (chromosomes.len() as f32 * nongraded_retain_percent) as usize;
    let selected = chromosomes[0..keep_count].to_vec();
    let mut rng = thread_rng();

    let nongraded_selected = chromosomes[keep_count..]
        .choose_multiple(&mut rng, keep_ungraded_count)
        .map(ToOwned::to_owned)
        .collect();

    let selected = [selected, nongraded_selected].concat();

    selected
}

pub fn mutate(chromosone: &str) -> String {
    let mut rng = thread_rng();
    let random_character = rng.sample(Alphanumeric) as char;
    let index = rng.gen_range(0..chromosone.len());
    let mut characters = chromosone.chars().collect::<Vec<char>>();

    characters[index] = random_character;
    characters.into_iter().collect()
}

pub fn crossover(chromosone_one: &str, chromosone_two: &str) -> String {
    let length = chromosone_one.len() / 2;

    format!(
        "{}{}",
        &chromosone_one[0..length],
        &chromosone_two[length..]
    )
}

pub fn create_population(size: usize, chromosone_size: usize) -> Vec<String> {
    let mut population = vec![];
    for _ in 0..size {
        population.push(create_chromosome(chromosone_size));
    }

    population
}

pub fn generation(
    mut population: Vec<String>,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    answer: &str,
) -> Vec<String> {
    let mut survivors = selection(
        &mut population,
        graded_retain_percent,
        nongraded_retain_percent,
        answer,
    );
    let mut children = vec![];
    let mut rng = thread_rng();

    while (children.len() + survivors.len()) < population.len() {
        let parent_1 = survivors.choose(&mut rng).unwrap();
        let parent_2 = survivors.choose(&mut rng).unwrap();
        let mut child = crossover(parent_1, &parent_2);
        let mutate_chance = rng.gen_range(0..1000);
        if mutate_chance <= 1 {
            child = mutate(&child);
        }
        children.push(child);
    }

    survivors.extend(children);

    survivors
}
