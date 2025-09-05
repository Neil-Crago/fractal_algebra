//! Defines `FractalEdge`, an abstract representation of a connection or interaction.
//!
//! Unlike the geometric `GraphEdge`, a `FractalEdge` represents a more computational
//! or signal-processing concept, defined by its amplitude, a discrete location, and phase.

use num_complex::Complex32; // A common alias for Complex<f32>

/// A type alias for the complex numbers used as scalars in this module.
pub type Scalar = Complex32;
use crate::constants::MODULUS;

/// Represents an abstract edge in a computational or signal-based system.
#[derive(Clone, Debug, PartialEq)]
pub struct FractalEdge {
    /// The strength or magnitude of the signal, represented by a complex number.
    pub amplitude: Scalar,
    /// A discrete index or address, often used in modular arithmetic.
    pub location: usize,
    /// The phase angle of the signal in radians.
    pub phase: f32,
}

impl FractalEdge {
    /// Performs a convolution of two edges.
    ///
    /// This operation combines the edges multiplicatively and adds their locations
    /// and phases, wrapping the location by the global `MODULUS`. It's analogous
    /// to frequency-domain multiplication in signal processing.
    pub fn convolve(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude * other.amplitude,
            location: (self.location + other.location) % MODULUS,
            phase: self.phase + other.phase, // Note: phase is not wrapped, may accumulate.
        }
    }

    /// Combines two edges through constructive interference.
    ///
    /// This operation adds the amplitudes of two edges. It requires that the edges
    /// be "in-phase" to be physically meaningful, a condition which is strictly
    /// enforced by an assertion.
    ///
    /// # Panics
    /// Panics if the absolute difference between the phases of `self` and `other`
    /// is greater than a small epsilon (1e-3).
    pub fn combine(&self, other: &Self) -> Self {
        // This assertion ensures that we only combine edges that are coherent (in-phase).
        // In a production library, this might be a `Result` instead of a panic.
        assert!((self.phase - other.phase).abs() < 1e-3, "Phases must match for combination");

        FractalEdge {
            amplitude: self.amplitude + other.amplitude,
            location: self.location, // Location is preserved from `self`.
            phase: self.phase,
        }
    }
}