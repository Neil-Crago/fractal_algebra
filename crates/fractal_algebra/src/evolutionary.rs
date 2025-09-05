//! Defines a `Generator` that uses an evolutionary strategy to create new `FractalField` candidates.
//!
//! This generator is a core component of a genetic or evolutionary algorithm. It produces
//! new fields by applying a suite of mutations to an existing field.

use crate::field::FractalField;
use crate::mutation::MutationSuite;
use crate::traits::Generator;

/*
Note: The commented-out code below represents a potential future refactor
where the `EvolutionaryGenerator` could be made generic over a `MutationStrategy`.
This would allow for easily swapping different mutation algorithms.

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

/// A generator that creates a population of `FractalField`s by mutating a parent field.
pub struct EvolutionaryGenerator {
    /// The suite of mutation operations to apply.
    pub mutations: MutationSuite,
    /// The number of new candidates to generate in each `mutate` call.
    pub count: usize,
}

impl Generator for EvolutionaryGenerator {
    /// Generates the initial seed population.
    /// In this implementation, it's a single "identity" field to kickstart the process.
    fn generate(&self) -> Vec<FractalField> {
        vec![FractalField::one()]
    }

    /// Creates a new generation of fields by mutating a given parent field.
    /// It applies the `MutationSuite` `count` times to produce a new population.
    fn mutate(&self, field: &FractalField) -> Vec<FractalField> {
        (0..self.count)
            .map(|_| self.mutations.mutate(field))
            .collect()
    }
}