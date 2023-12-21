use glam::Vec2;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::game_info::GameInfo;

#[derive(Debug, Clone, Copy)]
pub struct Individual {
    pub aim: Vec2,
    pub score: f32,
}

impl Individual {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let aim_x = rng.gen();
        let aim_y = rng.gen();
        let aim = Vec2::new(aim_x, aim_y);
        let score = 0.0;

        Self { aim, score }
    }

    pub fn new_from_parents(
        parent_one: &Self,
        parent_two: &Self,
        mutation_chance: f32,
        rng: &mut ThreadRng,
    ) -> Self {
        let aim = Vec2::new(parent_one.aim.x, parent_two.aim.y);
        let mut individual = Self { aim, score: 0.0 };

        if rng.gen::<f32>() < mutation_chance {
            individual.mutate(rng);
        }

        individual
    }

    pub fn update(&mut self, game_info: GameInfo) {
        let starting_distance = (game_info.position - game_info.target_position)
            .length()
            .abs();
        let mut bullet_distance_to_target = (game_info.bullet_position - game_info.target_position)
            .length()
            .abs();

        if bullet_distance_to_target <= game_info.target_size {
            bullet_distance_to_target = game_info.target_size
        }

        let score = game_info.target_size / bullet_distance_to_target;
        if score > self.score {
            self.score = score;
        }
    }

    pub fn play(&self, game_info: &GameInfo) -> Vec2 {
        self.aim
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        let aim_x = rng.gen();
        let aim_y = rng.gen();

        self.aim = Vec2::new(aim_x, aim_y);
    }
}
