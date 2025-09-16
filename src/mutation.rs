//! Defines a system for applying weighted, stochastic mutations to a `FractalField`.
//!
//! The main component is `MutationSuite`, which holds a collection of different
//! `MutationStrategy` implementations. When asked to mutate a field, it picks one
//! of these strategies based on a weighted random choice.

use crate::field::FractalField;
use crate::traits::MutationStrategy;
use rand::Rng;

/// A collection of weighted mutation strategies.
///
/// This struct allows for combining multiple mutation algorithms into a single operation.
/// The probability of any given strategy being chosen is proportional to its assigned weight.
#[derive(Default)]
pub struct MutationSuite {
    /// A vector of tuples, each containing a boxed `MutationStrategy` and its `f32` weight.
    pub strategies: Vec<(Box<dyn MutationStrategy>, f32)>,
}

impl MutationSuite {
    /// Creates a new, empty `MutationSuite`.
    pub fn new() -> Self {
        MutationSuite { strategies: Vec::new() }
    }

    /// Adds a new mutation strategy to the suite with a specified weight.
    ///
    /// # Type Parameters
    ///
    /// * `S`: A type that implements `MutationStrategy` and has a static lifetime.
    pub fn add_strategy<S: MutationStrategy + 'static>(&mut self, strategy: S, weight: f32) {
        self.strategies.push((Box::new(strategy), weight));
    }

    /// Mutates the given `FractalField` by selecting and applying one of the strategies.
    ///
    /// The selection is random, with the chance of each strategy being picked determined
    /// by its weight relative to the total weight of all strategies.
    ///
    /// If no strategies are present, it returns a clone of the original field as a fallback.
    pub fn mutate(&self, field: &FractalField) -> FractalField {
        if self.strategies.is_empty() {
            return field.clone(); // Fallback if no strategies are added.
        }

        let mut rng = rand::rng();

        // Calculate the sum of all weights to define the range for the random choice.
        let total_weight: f32 = self.strategies.iter().map(|(_, w)| *w).sum();

        // Pick a random value within that range.
        let mut choice = rng.random_range(0.0..total_weight);

        // Iterate through the strategies, subtracting their weight from the choice
        // until the choice is less than the current strategy's weight.
        for (strategy, weight) in &self.strategies {
            if choice < *weight {
                return strategy.mutate(field);
            }
            choice -= weight;
        }

        // This fallback should theoretically not be reached if total_weight > 0,
        // but it's good practice for robustness.
        field.clone()
    }
}