use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
pub struct Perceptron {
    pub weights: Vec<f32>,
    pub input_count: usize,
}

impl Perceptron {
    pub fn new(rng: &mut ThreadRng, input_count: usize) -> Self {
        let weights = vec![rng.gen_range(-1.0..=1.0); input_count];

        Self {
            weights,
            input_count,
        }
    }

    pub fn new_from_parents(parent_one: &Self, parent_two: &Self) -> Self {
        let input_count = parent_one.input_count;
        let mut weights = vec![];
        let half_input_count = input_count / 2;

        weights.extend_from_slice(&parent_one.weights[0..half_input_count]);
        weights.extend_from_slice(&parent_two.weights[half_input_count..]);

        Self {
            weights,
            input_count,
        }
    }

    pub fn guess(&self, inputs: &[f32]) -> i8 {
        let mut sum = 0.0;

        for (index, weight) in self.weights.iter().enumerate() {
            sum += inputs[index] * weight;
        }

        Self::activate(sum)
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        let random_index = rng.gen_range(0..self.input_count);
        let random_weight = rng.gen_range(-1.0..=1.0);

        self.weights[random_index] = random_weight;
    }

    fn activate(sum: f32) -> i8 {
        if sum > 0.5 {
            1
        } else if sum < -0.5 {
            -1
        } else {
            0
        }
    }
}
