use crate::state::{EntropyPulse, FeedbackSignal};

/// Defines the behavior of our Bayesian learning engine.
pub trait ProbabilisticSearch {
    fn propose_best_guess(&self) -> EntropyPulse;
    // Add the last_guess parameter here
    fn update(&mut self, feedback: &FeedbackSignal, last_guess: &EntropyPulse); // <-- Now 3 parameters
}
/// Defines the behavior of our symmetry constraint engine.
pub trait SymmetryConstraint {
    /// Check if a proposed pulse would violate the required symmetry.
    fn is_valid(&self, pulse: &EntropyPulse) -> bool;
}
