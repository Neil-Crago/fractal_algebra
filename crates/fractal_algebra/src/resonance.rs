//! Defines the core concepts of Resonance for the Fractal Algebra library.
//!
//! "Resonance" is an abstraction used to measure and classify patterns of coherence,
//! harmony, or structural integrity within and between fractal objects. This module provides
//! the `Resonance` trait, implementations for various types, and tools for transforming
//! and filtering objects based on their resonant properties.

use crate::fractaledge::FractalEdge;
use crate::signature::FractalSignature;
use crate::traits::{Fractal, FractalQuantumSpace};
use std::any::Any;
use std::f32::consts::PI;

/// A trait for objects that have a measurable, classifiable resonance.
///
/// Resonance may reflect phase alignment, recursive coherence, entropy modulation,
/// or structural harmony across transformations. This trait must be object-safe,
/// meaning its methods must work on `&dyn Resonance` trait objects.
pub trait Resonance {
    /// A helper method to allow for runtime type checking (downcasting).
    /// This is essential for methods like `resonance_similarity` that need to know
    /// the concrete type of the `other` object.
    fn as_any(&self) -> &dyn Any;

    /// Returns a scalar score representing the object's intrinsic resonance.
    /// Higher scores typically mean more "resonant" or coherent states.
    fn resonance_score(&self) -> f64;

    /// Compares the resonance of this object with another.
    ///
    /// # Returns
    /// A similarity score in the range `[0.0, 1.0]`, where 1.0 means perfect resonance.
    /// Implementations for concrete types should use `as_any()` to downcast `other`
    /// and check if they are comparing compatible types.
    fn resonance_similarity(&self, other: &dyn Resonance) -> f64;

    /// Returns `true` if the resonance similarity with another object exceeds a threshold.
    fn is_resonant_with(&self, other: &dyn Resonance) -> bool {
        self.resonance_similarity(other) > 0.8
    }

    /// Classifies the resonance pattern or law governing this object.
    fn resonance_law(&self) -> ResonanceLaw;

    /// Optionally returns a numeric signature or fingerprint of the resonance.
    /// Useful for entropy engines, machine learning, or transform matching.
    fn resonance_signature(&self) -> Option<Vec<f64>> {
        None
    }
}

/// A philosophical or structural classification of a resonance pattern.
///
/// This enum is used to describe the nature of alignment, coherence, or divergence
/// in fractal, transformational, or recursive systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceLaw {
    /// Self-similarity or recursive echoing.
    Echo,
    /// Self-similar expansion or growth patterns.
    FractalGrowth,
    /// Constructive interference or phase alignment.
    Harmony,
    /// Destructive interference or phase misalignment.
    Dissonance,
    /// Modulated divergence, often factorial or entropic.
    EntropyPulse,
    /// Resonance preserved under transformation.
    Invariant,
    /// Quasi-periodic or emergent resonance from chaotic systems.
    ChaoticBeat,
    /// No detectable resonance; flat or noise-like.
    Null,
    /// Custom or domain-specific resonance not yet classified.
    Other(&'static str),
}

impl std::fmt::Display for ResonanceLaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            ResonanceLaw::Echo => "Echo (recursive self-similarity)",
            ResonanceLaw::FractalGrowth => "Fractal Growth (self-similar expansion)",
            ResonanceLaw::Harmony => "Harmony (constructive alignment)",
            ResonanceLaw::Dissonance => "Dissonance (destructive misalignment)",
            ResonanceLaw::EntropyPulse => "Entropy Pulse (modulated divergence)",
            ResonanceLaw::Invariant => "Invariant (transform-preserving)",
            ResonanceLaw::ChaoticBeat => "Chaotic Beat (emergent quasi-periodicity)",
            ResonanceLaw::Null => "Null (no resonance)",
            ResonanceLaw::Other(label) => return write!(f, "Other ({})", label),
        };
        write!(f, "{}", description)
    }
}

/// A blanket implementation of `Resonance` for any type that has a `Default` and is `'static`.
/// This provides a non-resonant baseline behavior for any type that hasn't defined its own.
impl<T: Default + 'static> Resonance for T {
    fn as_any(&self) -> &dyn Any { self }
    fn resonance_score(&self) -> f64 { 0.0 }
    fn resonance_similarity(&self, _other: &dyn Resonance) -> f64 { 0.0 }
    fn resonance_law(&self) -> ResonanceLaw { ResonanceLaw::Null }
}

/// An implementation of `Resonance` for the abstract `FractalEdge` type.
impl Resonance for FractalEdge {
    fn as_any(&self) -> &dyn Any { self }

    fn resonance_score(&self) -> f64 {
        // Score is based on amplitude and how closely the phase aligns with integer multiples of 2π.
        let amp = self.amplitude.norm() as f64;
        let phase_alignment = (1.0 - (self.phase % (2.0 * PI)).cos() as f64).abs();
        amp * phase_alignment
    }


    fn resonance_similarity(&self, other: &dyn Resonance) -> f64 {
        // Use `as_any()` and `downcast_ref` to safely check if `other` is also a `FractalEdge`.
        if let Some(other_edge) = other.as_any().downcast_ref::<FractalEdge>() {
            // If the types match, we can compare their properties.
            let phase_diff = (self.phase - other_edge.phase).abs();
            let phase_similarity = 1.0 - (phase_diff % (2.0 * PI)).cos().abs() as f64;

            let amp_self = self.amplitude.norm() as f64;
            let amp_other = other_edge.amplitude.norm() as f64;
            let amp_ratio = if amp_self.max(amp_other) == 0.0 {
                1.0
            } else {
                amp_self.min(amp_other) / amp_self.max(amp_other)
            };

            // Combine the similarities with a weighted average.
            0.6 * phase_similarity + 0.4 * amp_ratio
        } else {
            // If `other` is not a `FractalEdge`, they have no similarity.
            0.0
        }
    }

    fn resonance_law(&self) -> ResonanceLaw {
        let amp = self.amplitude.norm();
        let phase = self.phase % (2.0 * PI);

        if amp < 0.01 { ResonanceLaw::Null }
        else if phase.abs() < 0.1 { ResonanceLaw::Harmony }
        else if (phase - PI).abs() < 0.1 { ResonanceLaw::Dissonance }
        else if amp > 10.0 { ResonanceLaw::EntropyPulse }
        else { ResonanceLaw::Echo }
    }

    fn resonance_signature(&self) -> Option<Vec<f64>> {
        Some(vec![
            self.amplitude.re as f64,
            self.amplitude.im as f64,
            self.phase as f64,
            self.location as f64,
        ])
    }
}

/// A trait for transformations that can affect an object's resonance.
pub trait ResonantTransform<T: Resonance> {
    /// Applies the transformation to a resonant object, returning the new state.
    fn apply(&self, input: &T) -> T;

    /// Calculates the change in resonance score after applying the transformation.
    fn resonance_delta(&self, input: &T) -> f64 {
        let before = input.resonance_score();
        let after = self.apply(input).resonance_score();
        after - before
    }

    /// Classifies the effect of the transformation on resonance.
    fn transform_law(&self, input: &T) -> TransformResonanceLaw {
        let delta = self.resonance_delta(input);
        if delta.abs() < 0.01 { TransformResonanceLaw::Invariant }
        else if delta > 0.0 { TransformResonanceLaw::Amplifying }
        else { TransformResonanceLaw::Dampening }
    }
}

/// Classifies the effect a `ResonantTransform` has on an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformResonanceLaw {
    Invariant,
    Amplifying,
    Dampening,
    Chaotic,
}

/// A simple transformation that shifts the phase of a `FractalEdge`.
pub struct PhaseShift {
    pub delta: f32,
}

impl ResonantTransform<FractalEdge> for PhaseShift {
    fn apply(&self, input: &FractalEdge) -> FractalEdge {
        FractalEdge { phase: input.phase + self.delta, ..*input }
    }
}

/// A filter that selects objects based on resonance criteria.
pub trait ResonanceFilter {
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit>;
    fn passes(&self, fractal: &dyn Fractal) -> bool;
    fn as_any(&self) -> &dyn Any;
}

/// A symbolic representation of a quantum-like fragment of information.
#[derive(Clone, Debug)]
pub struct SemanticUnit {
    pub label: String,
    pub depth: usize,
    pub phase: f64,
    pub fractal: Box<dyn Fractal>,
}

impl PartialEq for SemanticUnit {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.depth == other.depth
            && self.phase == other.phase
            && self.fractal.is_equal(&*other.fractal) // `is_equal` for trait objects
    }
}

impl Eq for SemanticUnit {}

/// A rule that transforms a semantic unit into deeper structure
pub struct ResonanceRule {
    pub transformation: fn(&SemanticUnit) -> Vec<SemanticUnit>,
}

/// A collection of `SemanticUnit`s forming a computational space.
pub struct SemanticLattice {
    pub units: Vec<SemanticUnit>,
    pub depth: usize,
}

impl FractalQuantumSpace for SemanticLattice {
    type SemanticUnit = SemanticUnit;

    fn resonance_depth(&self) -> usize { self.depth }

    fn project(&self, depth: usize) -> Vec<Self::SemanticUnit> {
        self.units
            .iter()
            .filter(|u| u.depth == depth)
            .cloned()
            .collect()
    }

    fn transform(&mut self, rule: &ResonanceRule) {
        let new_units: Vec<SemanticUnit> = self
            .units
            .iter()
            .filter(|u| u.depth == self.depth)
            .flat_map(|u| (rule.transformation)(u))
            .collect();
        self.units.extend(new_units);
        self.depth += 1;
    }

    fn filter(&self, filter: &dyn ResonanceFilter) -> Vec<Self::SemanticUnit> {
        filter.apply(&self.units)
    }

    fn fractal_signature(&self) -> FractalSignature {
        FractalSignature::from_units(&self.units)
    }
}
