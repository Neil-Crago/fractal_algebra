use crate::field::FractalField;
use crate::mutation::MutationSuite;
use crate::traits::Generator;

/*
pub struct EvolutionaryGenerator<S: MutationStrategy> {
    pub strategy: S,
    pub count: usize,
}

impl<S: MutationStrategy> Generator for EvolutionaryGenerator<S> {
    fn generate(&self) -> Vec<FractalField> {
        // Initial random generation or seed-based
        vec![FractalField::one()]
    }

    fn mutate(&self, field: &FractalField) -> Vec<FractalField> {
        (0..self.count)
            .map(|_| self.strategy.mutate(field))
            .collect()
    }
}
*/

pub struct EvolutionaryGenerator {
    pub mutations: MutationSuite,
    pub count: usize,
}

impl Generator for EvolutionaryGenerator {
    fn generate(&self) -> Vec<FractalField> {
        vec![FractalField::one()]
    }

    fn mutate(&self, field: &FractalField) -> Vec<FractalField> {
        (0..self.count)
            .map(|_| self.mutations.mutate(field))
            .collect()
    }
}
