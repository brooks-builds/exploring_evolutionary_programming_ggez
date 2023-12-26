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

    pub fn guess(&self, inputs: &[f32]) -> i8 {
        let mut sum = 0.0;

        for (index, weight) in self.weights.iter().enumerate() {
            sum += inputs[index] * weight;
        }

        Self::activate(sum)
    }

    fn activate(sum: f32) -> i8 {
        if sum > 0.1 {
            1
        } else if sum < -0.1 {
            -1
        } else {
            0
        }
    }
}
