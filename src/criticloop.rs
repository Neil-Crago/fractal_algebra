//! Provides a generator-critic evolutionary loop for finding optimal `FractalField`s.
//!
//! This module defines `GeneratorCriticLoop`, a structure that repeatedly uses a `Generator`
//! to create candidate solutions and a `CriticSuite` to evaluate and select the best one,
//! iteratively improving the solution over a set number of iterations.

use crate::critics::CriticSuite;
use crate::field::FractalField;
use crate::looprep::LoopReport;
use crate::traits::Generator;

/// Manages an evolutionary loop that generates and evaluates `FractalField` candidates.
pub struct GeneratorCriticLoop<G: Generator> {
    /// The generator responsible for creating initial and mutated `FractalField`s.
    pub generator: G,
    /// The suite of critics used to score and select the best candidates.
    pub critic_suite: CriticSuite,
    /// The total number of iterations to run the loop.
    pub iterations: usize,
}

impl<G: Generator> GeneratorCriticLoop<G> {
    /// Runs the evolutionary loop and returns the best `FractalField` found.
    ///
    /// # Returns
    ///
    /// An `Option<FractalField>` containing the best field if one was found, or `None`
    /// if the generator produced no candidates.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming Generator, CriticSuite, etc. are defined and in scope.
    /// /*
    /// let generator = RandomFieldGenerator { count: 10 };
    /// let mut suite = CriticSuite::new();
    /// suite.add_critic(SymmetryCritic, 0.5);
    /// suite.add_critic(EntropyCritic, 0.5);
    ///
    /// let loop_engine = GeneratorCriticLoop {
    ///     generator,
    ///     critic_suite: suite,
    ///     iterations: 20,
    /// };
    ///
    /// if let Some(best_field) = loop_engine.run() {
    ///     println!("Best field found with score: {}", loop_engine.critic_suite.score(&best_field));
    /// }
    /// */
    /// ```
    pub fn run(&self) -> Option<FractalField> {
        let mut best_field: Option<FractalField> = None;

        for _ in 0..self.iterations {
            // Generate new candidates, either from scratch or by mutating the current best.
            let candidates = match &best_field {
                Some(f) => self.generator.mutate(f),
                None => self.generator.generate(),
            };

            // Find the best candidate from the new batch.
            if let Some(best_candidate) = self.critic_suite.select_best(&candidates) {
                // Check if this candidate is better than our overall best so far.
                let is_improvement = match &best_field {
                    // If we have a current best, compare scores.
                    Some(current_best) => self.critic_suite.score(best_candidate) > self.critic_suite.score(current_best),
                    // If we don't have a best yet, any candidate is an improvement.
                    None => true,
                };

                if is_improvement {
                    best_field = Some(best_candidate.clone());
                }
            }
        }

        best_field
    }

    /// Runs the loop and returns a full report including the history of candidates.
    ///
    /// This method is useful for analysis and debugging, as it tracks every
    /// candidate that was considered the best in its generation, along with its score.
    pub fn run_with_report(&self) -> Option<LoopReport> {
        let mut best_field: Option<FractalField> = None;
        let mut history = Vec::new();

        for _ in 0..self.iterations {
            let candidates = match &best_field {
                Some(f) => self.generator.mutate(f),
                None => self.generator.generate(),
            };

            if let Some(best_candidate) = self.critic_suite.select_best(&candidates) {
                let score = self.critic_suite.score(best_candidate);
                history.push((best_candidate.clone(), score));

                let is_improvement = match &best_field {
                    Some(current_best) => score > self.critic_suite.score(current_best),
                    None => true,
                };

                if is_improvement {
                    best_field = Some(best_candidate.clone());
                }
            }
        }

        // If a best field was found, construct the report.
        best_field.map(|f| LoopReport {
            best_score: self.critic_suite.score(&f),
            best_field: f, // f has been moved, so no clone needed here.
            history,
        })
    }
}