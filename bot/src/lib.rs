mod individual;
mod logic;

use logic::{create_population, generation, Population};

pub struct Bot {
    winning_position: f32,
    population: Population,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    pub generation_count: u64,
    mutation_chance: f32,
}

impl Bot {
    pub fn new() -> Self {
        let winning_position = 1150.0_f32;
        let population_size = 10;
        let population = create_population(population_size);
        let graded_retain_percent = 0.3;
        let nongraded_retain_percent = 0.2;
        let generation_count = 1;
        let mutation_chance = 0.01;

        Self {
            winning_position,
            population,
            graded_retain_percent,
            nongraded_retain_percent,
            generation_count,
            mutation_chance,
        }
    }

    pub fn play(&self) -> Vec<f32> {
        self.population
            .iter()
            .map(|individual| individual.force_x)
            .collect()
    }

    pub fn run(&mut self, close_to_edges: Vec<f32>) {
        self.population
            .iter_mut()
            .zip(close_to_edges)
            .for_each(|(individual, close_to_edge)| individual.end_position_x = close_to_edge);

        self.population
            .iter_mut()
            .for_each(|individual| individual.set_score(self.winning_position));

        let successful: Vec<&individual::Individual> = self
            .population
            .iter()
            .filter(|individual| individual.score > 0.9999)
            .collect();

        if !successful.is_empty() {
            println!(
                "an individual figured out the correct force after {} generations: {:?}",
                self.generation_count, successful
            );
            std::process::exit(0);
        }

        let population = self.population.clone();
        self.population = generation(
            population,
            self.graded_retain_percent,
            self.nongraded_retain_percent,
            self.mutation_chance,
        );

        self.generation_count += 1;
    }
}
