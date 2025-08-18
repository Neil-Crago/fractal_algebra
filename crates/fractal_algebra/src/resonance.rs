use std::any::Any; // ADDED: Import the Any trait for downcasting
use std::f32::consts::PI;
use crate::fractaledge::FractalEdge;
use crate::signature::FractalSignature;
use crate::traits::{Fractal, FractalCollection, FractalQuantumSpace, Operation, CollectionMember};

/// Trait representing the resonance of a mathematical or computational object.
/// Resonance may reflect phase alignment, recursive coherence, entropy modulation,
/// or structural harmony across transformations.
pub trait Resonance {
    // ADDED: A helper method to allow for dynamic type checking (downcasting).
    fn as_any(&self) -> &dyn Any;

    /// Returns a scalar score representing intrinsic resonance.
    /// This could be based on phase coherence, symmetry, entropy, etc.
    fn resonance_score(&self) -> f64;

    /// Compares resonance with another object.
    /// Returns a similarity score in [0.0, 1.0], where 1.0 means perfect resonance.
    // CHANGED: The signature now takes a trait object `&dyn Resonance` to be object-safe.
    fn resonance_similarity(&self, other: &dyn Resonance) -> f64;

    /// Returns true if resonance similarity exceeds a threshold.
    // CHANGED: This now also takes a trait object to match the method above.
    fn is_resonant_with(&self, other: &dyn Resonance) -> bool {
        self.resonance_similarity(other) > 0.8
    }

    /// Classifies the resonance pattern or law governing this object.
    fn resonance_law(&self) -> ResonanceLaw;

    /// Optionally returns a signature or fingerprint of resonance.
    /// Useful for entropy engines or transform matching.
    fn resonance_signature(&self) -> Option<Vec<f64>> {
        None
    }
}

/// Philosophical or structural classification of resonance.
/// Used to describe the nature of alignment, coherence, or divergence
/// in fractal, transformational, or recursive systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceLaw {
    /// Self-similarity or recursive echoing.
    Echo,

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
        match self {
            ResonanceLaw::Echo => write!(f, "Echo (recursive self-similarity)"),
            ResonanceLaw::FractalGrowth => write!(f, "Fractal Growth (self-similar expansion)"),
            ResonanceLaw::Harmony => write!(f, "Harmony (constructive alignment)"),
            ResonanceLaw::Dissonance => write!(f, "Dissonance (destructive misalignment)"),
            ResonanceLaw::EntropyPulse => write!(f, "Entropy Pulse (modulated divergence)"),
            ResonanceLaw::Invariant => write!(f, "Invariant (transform-preserving)"),
            ResonanceLaw::ChaoticBeat => write!(f, "Chaotic Beat (emergent quasi-periodicity)"),
            ResonanceLaw::Null => write!(f, "Null (no resonance)"),
            ResonanceLaw::Other(label) => write!(f, "Other ({})", label),
        }
    }
}


// This default implementation will need to be updated to match the new trait definition.
impl<T> Resonance for T
where
    T: Default + 'static, // ADDED: 'static bound is needed for as_any
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn resonance_score(&self) -> f64 {
        0.0
    }

    fn resonance_similarity(&self, _other: &dyn Resonance) -> f64 {
        0.0
    }

    fn resonance_law(&self) -> ResonanceLaw {
        ResonanceLaw::Dissonance
    }
}


impl Resonance for FractalEdge {
    // ADDED: Implementation of the required as_any method.
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn resonance_score(&self) -> f64 {
        // Score = amplitude magnitude × phase alignment factor
        let amp = self.amplitude.norm() as f64;
        let phase_alignment = (1.0 - (self.phase % (2.0 * PI)).cos() as f64).abs();
        amp * phase_alignment
    }

    // CHANGED: This method is now implemented for the object-safe signature.
    fn resonance_similarity(&self, other: &dyn Resonance) -> f64 {
        // We must first try to downcast the other trait object to our concrete type.
        if let Some(other_edge) = other.as_any().downcast_ref::<FractalEdge>() {
            // If the downcast succeeds, the types are the same, and we can run our logic.
            // Phase similarity: cosine distance
            let phase_diff = (self.phase - other_edge.phase).abs();
            let phase_similarity = 1.0 - (phase_diff % (2.0 * PI)).cos().abs() as f64;

            // Amplitude similarity: ratio of magnitudes
            let amp_self = self.amplitude.norm() as f64;
            let amp_other = other_edge.amplitude.norm() as f64;
            let amp_ratio = if amp_self.max(amp_other) == 0.0 {
                1.0
            } else {
                amp_self.min(amp_other) / amp_self.max(amp_other)
            };

            // Weighted average
            0.6 * phase_similarity + 0.4 * amp_ratio
        } else {
            // If the downcast fails, the types are different. They cannot be resonant.
            0.0
        }
    }

    fn resonance_law(&self) -> ResonanceLaw {
        let amp = self.amplitude.norm();
        let phase = self.phase % (2.0 * PI);

        match (amp, phase) {
            (a, _p) if a < 0.01 => ResonanceLaw::Null,
            (_, p) if p.abs() < 0.1 => ResonanceLaw::Harmony,
            (_, p) if (p - PI).abs() < 0.1 => ResonanceLaw::Dissonance,
            (a, _) if a > 10.0 => ResonanceLaw::EntropyPulse,
            (_, _) => ResonanceLaw::Echo,
        }
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


/// Trait representing a transformation that affects resonance.
/// Used to model how fractal structures, signals, or recursive systems
/// change under spatial, phase, or entropy-based operations.
pub trait ResonantTransform<T: Resonance> {
    /// Applies the transformation to a resonant object.
    fn apply(&self, input: &T) -> T;

    /// Returns the resonance delta: how much the score changed.
    fn resonance_delta(&self, input: &T) -> f64 {
        let before = input.resonance_score();
        let after = self.apply(input).resonance_score();
        after - before
    }

    /// Classifies the effect of the transformation on resonance.
    fn transform_law(&self, input: &T) -> TransformResonanceLaw {
        let delta = self.resonance_delta(input);
        match delta {
            d if d.abs() < 0.01 => TransformResonanceLaw::Invariant,
            d if d > 0.0 => TransformResonanceLaw::Amplifying,
            d if d < 0.0 => TransformResonanceLaw::Dampening,
            _ => TransformResonanceLaw::Chaotic,
        }
    }

    /// Optionally returns a transformed resonance signature.
    fn transformed_signature(&self, input: &T) -> Option<Vec<f64>> {
        self.apply(input).resonance_signature()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformResonanceLaw {
    /// Resonance is preserved exactly.
    Invariant,

    /// Resonance is amplified or enhanced.
    Amplifying,

    /// Resonance is reduced or dampened.
    Dampening,

    /// Resonance is unpredictably altered.
    Chaotic,
}

pub struct PhaseShift {
    pub delta: f32,
}

impl ResonantTransform<FractalEdge> for PhaseShift {
    fn apply(&self, input: &FractalEdge) -> FractalEdge {
        FractalEdge {
            amplitude: input.amplitude,
            location: input.location,
            phase: input.phase + self.delta,
        }
    }
}

/// A sequence of resonance-aware transformations applied in order.
pub struct CompositeTransform<T, F>
where
    T: Resonance,
    F: ResonantTransform<T>,
{
    pub transforms: Vec<F>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> CompositeTransform<T, F>
where
    T: Resonance,
    F: ResonantTransform<T>,
{
    pub fn new(transforms: Vec<F>) -> Self {
        Self {
            transforms,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F> ResonantTransform<T> for CompositeTransform<T, F>
where
    T: Resonance + Clone,
    F: ResonantTransform<T>,
{
    fn apply(&self, input: &T) -> T {
        self.transforms.iter().fold(input.clone(), |acc, transform| {
            transform.apply(&acc)
        })
    }

    fn resonance_delta(&self, input: &T) -> f64 {
        let before = input.resonance_score();
        let after = self.apply(input).resonance_score();
        after - before
    }

    fn transform_law(&self, input: &T) -> TransformResonanceLaw {
        let delta = self.resonance_delta(input);
        match delta {
            d if d.abs() < 0.01 => TransformResonanceLaw::Invariant,
            d if d > 0.0 => TransformResonanceLaw::Amplifying,
            d if d < 0.0 => TransformResonanceLaw::Dampening,
            _ => TransformResonanceLaw::Chaotic,
        }
    }

    fn transformed_signature(&self, input: &T) -> Option<Vec<f64>> {
        self.apply(input).resonance_signature()
    }
}

/// A filter that selects semantic units based on resonance criteria.
pub trait ResonanceFilter {
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit>;
    fn passes(&self, fractal: &dyn Fractal) -> bool;
}

pub struct ResonanceRuleEngine {
    pub filters: Vec<Box<dyn ResonanceFilter>>,
    pub operation_rules: Vec<OperationRule>,
}

#[derive(Debug, Clone)]
pub struct OperationRule {
    pub operation: Operation,
    pub required_law: ResonanceLaw,
    pub min_score: f64,
}

impl ResonanceRuleEngine {
    pub fn is_operation_allowed(&self, member: &CollectionMember) -> bool {
        for rule in &self.operation_rules {
            if member.operation == rule.operation {
                let law_ok = member.fractal.resonance_law() == rule.required_law;
                let score_ok = member.fractal.resonance_score() >= rule.min_score;
                return law_ok && score_ok;
            }
        }
        true // No rule defined means allowed
    }

    pub fn validate_collection(&self, collection: &FractalCollection) -> Vec<(usize, bool)> {
        collection
            .members
            .iter()
            .enumerate()
            .map(|(i, member)| (i, self.is_operation_allowed(member)))
            .collect()
    }


pub fn diagnostics(&self, collection: &FractalCollection) -> Vec<String> {
    self.validate_collection(collection)
        .into_iter()
        .filter_map(|(i, allowed)| {
            if !allowed {
                Some(format!("Member {} violates resonance rule", i))
            } else {
                None
            }
        })
        .collect()
}
}


/*
// Usage
let engine = ResonanceRuleEngine {
    filters: vec![Box::new(LawFilter { allowed: vec![ResonanceLaw::Harmony] })],
    operation_rules: vec![
        OperationRule {
            operation: Operation::Union,
            required_law: ResonanceLaw::Harmony,
            min_score: 0.5,
        },
        OperationRule {
            operation: Operation::Intersection,
            required_law: ResonanceLaw::Invariant,
            min_score: 0.3,
        },
    ],
};
*/

/// A symbolic quantum fragment—could be a wavefunction, law, or transformation
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticUnit {
    pub label: String,
    pub depth: usize,
    pub phase: f64, // Optional: phase angle for resonance
}

/// A rule that transforms a semantic unit into deeper structure
pub struct ResonanceRule {
    pub transformation: fn(&SemanticUnit) -> Vec<SemanticUnit>,
}

/// A fractal quantum space implemented as a semantic lattice
pub struct SemanticLattice {
    pub units: Vec<SemanticUnit>,
    pub depth: usize,
}

impl FractalQuantumSpace for SemanticLattice {
    type SemanticUnit = SemanticUnit;

    fn resonance_depth(&self) -> usize {
        self.depth
    }

    fn project(&self, depth: usize) -> Vec<Self::SemanticUnit> {
        self.units.iter().filter(|u| u.depth == depth).cloned().collect()
    }

    fn transform(&mut self, rule: &ResonanceRule) {
        let new_units: Vec<SemanticUnit> = self.units
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
