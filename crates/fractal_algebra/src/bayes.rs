//! Implements a probabilistic search strategy using a Bayesian-like update rule.
//!
//! This module defines a `FrequencyBeliefSpace` which maintains a probabilistic model
//! (a set of Gaussian distributions) about the optimal parameters for an `EntropyPulse`.
//! It iteratively refines its beliefs to minimize an error metric from a `FeedbackSignal`.

use crate::ai::{EntropyPulse, FeedbackSignal, ProbabilisticSearch};
use rand_distr::{Distribution, Normal};

/// A simple Gaussian (Normal) distribution used to model a belief about a parameter.
/// The `mean` represents the current best guess, and `std_dev` represents the uncertainty
/// or the scope of exploration.
pub struct Gaussian {
    pub mean: f64,
    pub std_dev: f64,
}

/// Represents the AI's entire belief system about the target `EntropyPulse`.
///
/// It holds probability distributions for the pulse's frequency and amplitude,
/// and it remembers the best guess it has found so far. This memory is crucial
/// for ensuring the AI converges on the best solution it has seen.
pub struct FrequencyBeliefSpace {
    /// The belief distribution for the pulse's frequency.
    pub frequency: Gaussian,
    /// The belief distribution for the pulse's amplitude.
    pub amplitude: Gaussian,
    /// The best `EntropyPulse` found so far during the search.
    pub best_guess: EntropyPulse,
    /// The feedback signal corresponding to the `best_guess`, holding the smallest error.
    pub best_feedback: FeedbackSignal,
}

impl FrequencyBeliefSpace {
    /// Creates a new `FrequencyBeliefSpace` with initial guesses.
    ///
    /// The standard deviation for frequency is set high initially to encourage
    /// broad exploration of the problem space.
    pub fn new(initial_freq: f64, initial_amp: f64) -> Self {
        let initial_guess = EntropyPulse {
            frequency: initial_freq,
            amplitude: initial_amp,
            waveform: "sine".to_string(),
        };

        Self {
            frequency: Gaussian {
                mean: initial_freq,
                std_dev: 50.0, // Start with a wide search space for frequency.
            },
            amplitude: Gaussian {
                mean: initial_amp,
                std_dev: 1.0,
            },
            // Initialize memory with the initial guess.
            best_guess: initial_guess,
            // Initialize best feedback with the largest possible error, so any
            // real feedback will be considered an improvement.
            best_feedback: FeedbackSignal {
                correlation_strength: f64::MAX,
            },
        }
    }
}

impl ProbabilisticSearch for FrequencyBeliefSpace {
    /// Proposes a new `EntropyPulse` by sampling from the current belief distributions.
    ///
    /// This function represents the "exploration" phase. It generates a new guess
    /// based on the current mean (best belief) and standard deviation (uncertainty).
    fn propose_best_guess(&self) -> EntropyPulse {
        // It's safe to unwrap here because the std_dev is controlled internally
        // and is prevented from becoming non-positive in the update logic.
        let freq_dist = Normal::new(self.frequency.mean, self.frequency.std_dev).unwrap();
        let amp_dist = Normal::new(self.amplitude.mean, self.amplitude.std_dev).unwrap();
        let mut rng = rand::rng();

        EntropyPulse {
            frequency: freq_dist.sample(&mut rng),
            amplitude: amp_dist.sample(&mut rng),
            waveform: "sine".to_string(),
        }
    }

    /// Updates the belief space based on the feedback from the last guess.
    ///
    /// This is the core of the learning algorithm. It adjusts the mean of its
    /// beliefs to move closer to the best-known solution and reduces the
    /// standard deviation to narrow the search space over time (exploitation).
    fn update(&mut self, feedback: &FeedbackSignal, last_guess: &EntropyPulse) {
        // Step 1: Check if the latest guess is better than the best one found so far.
        // The goal is to minimize correlation_strength (error).
        if feedback.correlation_strength < self.best_feedback.correlation_strength {
            // We found a new best! Update our memory.
            self.best_feedback = feedback.clone();
            self.best_guess = last_guess.clone();
        }

        // Step 2: Update the belief mean.
        // Nudge the mean of our search distribution towards the best-known frequency.
        // This is a form of exponential moving average, which stabilizes learning.
        let learning_rate = 0.15; // A higher rate means we move faster towards the best guess.
        self.frequency.mean =
            (1.0 - learning_rate) * self.frequency.mean + learning_rate * self.best_guess.frequency;

        // Step 3: Reduce exploration over time (annealing).
        // Shrink the standard deviation to "zoom in" on the promising area.
        // This shifts the strategy from exploration to exploitation.
        self.frequency.std_dev *= 0.9;

        // Step 4: Prevent the search space from collapsing entirely.
        // A minimum standard deviation ensures the AI can always explore a little.
        if self.frequency.std_dev < 0.01 {
            self.frequency.std_dev = 0.01;
        }
    }
}