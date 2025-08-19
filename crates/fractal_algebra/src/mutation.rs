use crate::field::FractalField;
use crate::traits::MutationStrategy;

pub struct MutationSuite {
    pub strategies: Vec<(Box<dyn MutationStrategy>, f32)>, // (strategy, weight)
}

impl Default for MutationSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl MutationSuite {
    pub fn new() -> Self {
        MutationSuite {
            strategies: Vec::new(),
        }
    }

    pub fn add_strategy<S: MutationStrategy + 'static>(&mut self, strategy: S, weight: f32) {
        self.strategies.push((Box::new(strategy), weight));
    }

    pub fn mutate(&self, field: &FractalField) -> FractalField {
        use rand::Rng;
        let mut rng = rand::rng();

        let total_weight: f32 = self.strategies.iter().map(|(_, w)| w).sum();
        let mut choice = rng.random_range(0.0..total_weight);
        for (strategy, weight) in &self.strategies {
            if choice < *weight {
                return strategy.mutate(field);
            }
            choice -= weight;
        }

        field.clone() // fallback
    }
}
