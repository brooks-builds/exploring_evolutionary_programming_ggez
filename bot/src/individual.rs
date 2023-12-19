use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Individual {
    force_x: f32,
    end_position_x: f32,
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

    pub fn set_score(&mut self, target_x: f32) {
        self.score = self.end_position_x / target_x;
    }
}
