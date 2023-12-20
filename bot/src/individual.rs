use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Individual {
    pub force_x: f32,
    pub end_position_x: f32,
    pub score: f32,
}

impl Individual {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let force_x = rng.gen_range(0.0..100.0);
        let end_position_x = 0.0;
        let score = 0.0;

        Self {
            force_x,
            end_position_x,
            score,
        }
    }

    pub fn new_from_parents(
        parent_one: &Self,
        parent_two: &Self,
        mutation_chance: f32,
        rng: &mut ThreadRng,
    ) -> Self {
        let force_x_delta = (parent_one.force_x - parent_two.force_x).abs();
        let lowest_parent_force_x = parent_one.force_x.min(parent_two.force_x);
        let force_x = lowest_parent_force_x + (force_x_delta / 2.0);

        let mut individual = Self {
            force_x,
            end_position_x: 0.0,
            score: 0.0,
        };

        if rng.gen::<f32>() < mutation_chance {
            individual.mutate(rng);
        }

        individual
    }

    pub fn set_score(&mut self, target_x: f32) {
        self.score = self.end_position_x / target_x;
    }

    fn mutate(&mut self, rng: &mut ThreadRng) {
        self.force_x = rng.gen_range(0.0..100.0);
    }
}
