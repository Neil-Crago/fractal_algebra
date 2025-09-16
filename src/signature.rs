//! Defines `FractalSignature`, a condensed summary of a `FractalField`'s properties.

use crate::resonance::SemanticUnit;

/// A struct that holds aggregated data about a `FractalField`,
/// used for quick comparisons, classification, and evaluation by critics.
#[derive(Clone, Debug, PartialEq)]
pub struct FractalSignature {
    pub total_amplitude: f32,
    pub average_phase: f32,
    pub entropy: f32,
    pub edge_count: usize,
    pub depth_range: (u32, u32),
}

impl FractalSignature {
    /// Calculates a simple distance metric between two signatures.
    pub fn distance(&self, other: &Self) -> f32 {
        (self.total_amplitude - other.total_amplitude).abs()
        + (self.average_phase - other.average_phase).abs()
        + (self.entropy - other.entropy).abs()
        + (self.edge_count as i32 - other.edge_count as i32).abs() as f32
    }

    /// Checks if the signature is symmetric, defined as having an average phase
    /// close to 0 or π.
    pub fn is_symmetric(&self) -> bool {
        self.average_phase.abs() < 1e-3 || (self.average_phase.abs() - std::f32::consts::PI).abs() < 1e-3
    }

    /// Returns a new signature with its amplitude-dependent fields normalized.
    pub fn normalized(&self) -> Self {
        let amp = self.total_amplitude.max(1e-6); // Avoid division by zero
        FractalSignature {
            total_amplitude: 1.0,
            entropy: self.entropy / amp,
            ..*self
        }
    }

    /// Creates a `FractalSignature` from a slice of `SemanticUnit`s.
    pub fn from_units(units: &[SemanticUnit]) -> Self {
        if units.is_empty() {
            return FractalSignature {
                total_amplitude: 0.0,
                average_phase: 0.0,
                entropy: 0.0,
                edge_count: 0,
                depth_range: (0, 0),
            };
        }
        let count = units.len() as f32;
        let min_depth = units.iter().map(|u| u.depth).min().unwrap_or(0) as u32;
        let max_depth = units.iter().map(|u| u.depth).max().unwrap_or(0) as u32;

        FractalSignature {
            total_amplitude: units.iter().map(|u| u.depth as f32).sum(),
            average_phase: units.iter().map(|u| u.phase as f32).sum::<f32>() / count,
            entropy: units.iter().map(|u| u.depth as f32).sum::<f32>() / count, // Simple entropy proxy
            edge_count: units.len(),
            depth_range: (min_depth, max_depth),
        }
    }
}