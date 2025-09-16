//! Defines a `MutationStrategy` that applies stochastic jitter to amplitude and phase.

use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::traits::MutationStrategy;
use num_complex::Complex;
use rand::Rng;

/// A mutation strategy that perturbs the amplitude and phase of each edge's
/// complex data by a random amount.
pub struct StochasticAmplitudePhase {
    /// The maximum random change to apply to the amplitude.
    pub amplitude_jitter: f32,
    /// The maximum random change to apply to the phase (in radians).
    pub phase_jitter: f32,
}

impl MutationStrategy for StochasticAmplitudePhase {
    fn mutate(&self, field: &FractalField) -> FractalField {
        let mut rng = rand::rng();

        let edges = field
            .edges
            .iter()
            .map(|e| {
                // Deconstruct the complex number into polar coordinates (amplitude and phase)
                let (amp, phase) = e.data.to_polar();

                // Apply random jitter to each component
                let amp_mut = amp + rng.random_range(-self.amplitude_jitter..self.amplitude_jitter);
                let phase_mut = phase + rng.random_range(-self.phase_jitter..self.phase_jitter);

                GraphEdge {
                    // Reconstruct the complex data from the new polar coordinates
                    data: Complex::from_polar(amp_mut.max(0.0), phase_mut),
                    ..*e // Copy the rest of the fields from the original edge
                }
            })
            .collect();

        FractalField { edges }
    }
}