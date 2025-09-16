//! Defines the `LoopReport` struct for storing results from an evolutionary run.

use crate::field::FractalField;

/// A report containing the final results and history of a `GeneratorCriticLoop` run.
///
/// This struct is typically returned by the `run_with_report` method and is useful
/// for analysis, debugging, and visualizing the evolutionary process.
#[derive(Debug)]
pub struct LoopReport {
    /// The best `FractalField` found during the entire run.
    pub best_field: FractalField,
    /// The score of the `best_field`.
    pub best_score: f32,
    /// A history of the best candidate from each iteration, along with its score.
    pub history: Vec<(FractalField, f32)>,
}