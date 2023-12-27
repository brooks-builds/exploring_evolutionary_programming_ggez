use rand::{rngs::ThreadRng, Rng};

use crate::{command::Command, game_info::GameInfo, perceptron::Perceptron};

#[derive(Debug, Clone)]
pub struct Individual {
    rotate_perceptron: Perceptron,
    fire_perceptron: Perceptron,
    pub score: f32,
}

impl Individual {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let rotate_perceptron = Perceptron::new(rng, 7);
        let fire_perceptron = Perceptron::new(rng, 7);
        let score = 0.0;

        Self {
            rotate_perceptron,
            fire_perceptron,
            score,
        }
    }

    pub fn new_from_parents(
        parent_one: &Self,
        parent_two: &Self,
        mutation_chance: f32,
        rng: &mut ThreadRng,
    ) -> Self {
        let rotate_perceptron = parent_one.rotate_perceptron.clone();
        let fire_perceptron = parent_two.fire_perceptron.clone();
        let mut individual = Self {
            rotate_perceptron,
            fire_perceptron,
            score: 0.0,
        };

        if rng.gen_range(0.0..=1.0) < mutation_chance {
            individual.mutate(rng);
        }

        individual
    }

    pub fn update(&mut self, game_info: GameInfo) {
        let mut bullet_distance_to_target = (game_info
            .bullet_position
            .expect("must have bullet position")
            - game_info.target_position)
            .length()
            .abs();

        if bullet_distance_to_target <= game_info.target_size {
            bullet_distance_to_target = game_info.target_size
        }

        let score = game_info.target_size / bullet_distance_to_target; // 0..1 or -1..1
        if score > self.score {
            self.score = score;
        }
    }

    pub fn play(&self, game_info: &GameInfo) -> [Command; 2] {
        let position = game_info.position.normalize();
        let rotation = game_info.aim_rotation / 360.0;
        let target_position = game_info.target_position.normalize();
        let target_velocity = game_info.target_velocity.normalize();

        let inputs = [
            position.x,
            position.y,
            rotation,
            target_position.x,
            target_position.y,
            target_velocity.x,
            target_velocity.y,
        ];
        let rotation_guess = match self.rotate_perceptron.guess(&inputs) {
            1 => Command::RotateRight,
            -1 => Command::RotateLeft,
            0 => Command::Nothing,
            _ => unreachable!(),
        };
        let fire_guess = match self.rotate_perceptron.guess(&inputs) {
            1 => Command::Fire,
            _ => Command::Nothing,
        };

        [rotation_guess, fire_guess]
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        if rng.gen_bool(0.5) {
            self.rotate_perceptron = Perceptron::new(rng, 7);
        } else {
            self.fire_perceptron = Perceptron::new(rng, 7);
        }
    }
}
