use glam::Vec2;
use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::game_info::GameInfo;

#[derive(Debug, Clone, Copy)]
pub struct Individual {
    pub score: f32,
    pub aim_x: f32,
    pub aim_y: f32,
}

impl Individual {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let score = 0.0;
        let aim_x = rng.gen_range(-1.0..=1.0);
        let aim_y = rng.gen_range(-1.0..=1.0);

        Self {
            score,
            aim_x,
            aim_y,
        }
    }

    pub fn new_from_parents(
        parent_one: &Self,
        parent_two: &Self,
        mutation_chance: f32,
        rng: &mut ThreadRng,
    ) -> Self {
        let aim_x = parent_one.aim_x;
        let aim_y = parent_two.aim_y;
        let mut individual = Self {
            aim_x,
            aim_y,
            score: 0.0,
        };

        if rng.gen_range(0.0..1.0) < mutation_chance {
            individual.mutate(rng);
        }

        individual
    }

    pub fn update(&mut self, game_info: GameInfo) {
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
        let angle_between_us_and_target = game_info.position.angle_between(game_info.position);
        let angle =
            self.aim_x * angle_between_us_and_target + self.aim_y * game_info.target_velocity.y;
        Vec2::from_angle(angle) * game_info.bullet_speed
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        self.aim_x = rng.gen_range(-1.0..=1.0);
        self.aim_y = rng.gen_range(-1.0..=1.0);
    }
}
