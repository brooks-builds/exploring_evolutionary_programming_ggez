pub mod command;
pub mod game_info;
pub mod individual;
mod logic;
pub mod perceptron;

use command::Command;
use game_info::GameInfo;
use logic::{create_population, generation, Population};

pub struct Bot {
    pub population: Population,
    graded_retain_percent: f32,
    nongraded_retain_percent: f32,
    pub generation_count: u64,
    mutation_chance: f32,
    pub population_size: u8,
}

impl Bot {
    pub fn new(population_size: u8) -> Self {
        let population = create_population(population_size);
        let graded_retain_percent = 0.1;
        let nongraded_retain_percent = 0.05;
        let generation_count = 1;
        let mutation_chance = 0.25;

        Self {
            population,
            graded_retain_percent,
            nongraded_retain_percent,
            generation_count,
            mutation_chance,
            population_size,
        }
    }

    pub fn play(&self, game_info: GameInfo) -> Vec<[Command; 2]> {
        self.population
            .iter()
            .map(move |individual| individual.play(&game_info))
            .collect::<Vec<[Command; 2]>>()
    }

    pub fn run(&mut self) {
        // let successful: Vec<&individual::Individual> = self
        //     .population
        //     .iter()
        //     .filter(|individual| individual.score > 0.9999)
        //     .collect();

        // if !successful.is_empty() {
        //     println!(
        //         "an individual figured out the correct force after {} generations: {:?}",
        //         self.generation_count, successful
        //     );
        //     std::process::exit(0);
        // }

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
