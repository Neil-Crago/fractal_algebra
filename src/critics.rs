//! Defines a system for evaluating and scoring `FractalField` instances.
//!
//! The central component is the `CriticSuite`, which manages a collection of
//! weighted `Critic` trait objects. This allows for a flexible and composite
//` approach to evaluating generated fields based on multiple criteria.

use crate::field::FractalField;
use crate::traits::Critic;
use std::cmp::Ordering;

/// A collection of weighted critics to provide a composite score for a `FractalField`.
///
/// This struct allows for combining multiple evaluation criteria (e.g., symmetry, entropy)
/// into a single, weighted score. It can also select the best field from a slice of candidates.
#[derive(Default)]
pub struct CriticSuite {
    /// A vector of tuples, where each contains a boxed `Critic` trait object and its `f32` weight.
    pub critics: Vec<(Box<dyn Critic>, f32)>,
}

impl CriticSuite {
    /// Creates a new, empty `CriticSuite`.
    pub fn new() -> Self {
        CriticSuite {
            critics: Vec::new(),
        }
    }

    /// Adds a new critic to the suite with a specified weight.
    ///
    /// The weight determines the critic's influence on the final composite score.
    ///
    /// # Type Parameters
    ///
    /// * `C`: A type that implements the `Critic` trait and has a static lifetime.
    pub fn add_critic<C: Critic + 'static>(&mut self, critic: C, weight: f32) {
        self.critics.push((Box::new(critic), weight));
    }

    /// Calculates the total weighted score for a given `FractalField`.
    ///
    /// The score is the sum of `critic.score(field) * weight` for all critics in the suite.
    pub fn score(&self, field: &FractalField) -> f32 {
        self.critics
            .iter()
            .map(|(critic, weight)| critic.score(field) * weight)
            .sum()
    }

    /// Classifies a field based on the highest-weighted critic in the suite.
    ///
    /// This can be used to assign a primary "label" or "category" to a field.
    pub fn classify(&self, field: &FractalField) -> String {
        // Find the critic with the maximum weight.
        if let Some((critic, _)) = self.critics.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)) {
            critic.classify(&field.signature())
        } else {
            "unclassified".to_string()
        }
    }

    /// Selects the best `FractalField` from a slice of candidates based on the composite score.
    ///
    /// # Returns
    ///
    ///An `Option` containing a reference to the best field, or `None` if the input slice is empty.
    pub fn select_best<'a>(&self, fields: &'a [FractalField]) -> Option<&'a FractalField> {
        fields
            .iter()
            .max_by(|a, b| {
                let score_a = self.score(a);
                let score_b = self.score(b);
                score_a.partial_cmp(&score_b).unwrap_or(Ordering::Equal)
            })
    }
}