//! Defines the core traits and architectural patterns for the library.
//!
//! This module establishes the central abstractions for the entire system, creating a
//! clear separation between low-level algebra and high-level fractal composition.
//!
//! ## Core Concepts
//!
//! - **`VectorSpace` Trait**: Defines the basic mathematical properties for types that can be
//!   treated as vectors (e.g., `FractalField`). It relies on standard operator traits
//!   like `std::ops::Add` for idiomatic usage.
//!
//! - **`Fractal` Trait**: The primary trait for all high-level fractal objects. It provides a
//!   common, dynamically-dispatchable interface for things like `Mandelbrot` and `IFS`.
//!
//! - **`FractalType` Enum**: A type-safe enum that wraps all concrete `Fractal` implementations.
//!   This is the main type used for storing and combining different kinds of fractals.
//!
//! - **CSG Functions**: A set of functions (`add_fractals`, `sub_fractals`, etc.) that perform
//!   Constructive Solid Geometry-like operations on `FractalType`s, resulting in a
//!   `FractalCollection` that represents the combined object tree.

use crate::atom::{FractalAtom, Metadata, TagSet};
use crate::field::FractalField;
use crate::resonance::{ResonanceFilter, ResonanceLaw, ResonanceRule};
use crate::signature::FractalSignature;
use num_complex::Complex;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub, Neg, Mul};

// --- Low-Level Algebra ---

/// A trait for types that behave like elements in a vector space.
///
/// By requiring the standard operator traits as bounds, we ensure that any type
/// implementing `VectorSpace` can be used with `+`, `-`, and `*` in a predictable way.
pub trait VectorSpace:
    Sized
    + Clone
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Complex<f32>, Output = Self> // For scalar multiplication
{
    /// Returns the additive identity element (the "zero" vector).
    fn zero() -> Self;
}

/// A private, unused trait for defining algebraic law tests. Can be removed or implemented.
trait _AlgebraicLaws {
    fn test_commutativity(&self) -> bool;
    fn test_associativity(&self) -> bool;
}

// --- Generative System Traits ---

/// A trait for objects that can produce a condensed, descriptive signature.
pub trait HasSignature {
    fn signature(&self) -> FractalSignature;
}

impl HasSignature for FractalField {
    fn signature(&self) -> FractalSignature {
        if self.edges.is_empty() {
            return FractalSignature {
                total_amplitude: 0.0,
                average_phase: 0.0,
                entropy: 0.0,
                edge_count: 0,
                depth_range: (0, 0),
            };
        }

        let mut total_amp = 0.0;
        let mut total_phase = 0.0;
        let mut entropy = 0.0;
        let mut min_depth = u32::MAX;
        let mut max_depth = 0;

        for edge in &self.edges {
            let (amp, phase) = edge.data.to_polar();
            total_amp += amp;
            total_phase += phase;
            entropy += amp * phase.abs(); // crude entropy proxy
            min_depth = min_depth.min(edge.depth);
            max_depth = max_depth.max(edge.depth);
        }

        let count = self.edges.len() as f32;
        FractalSignature {
            total_amplitude: total_amp,
            average_phase: total_phase / count,
            entropy,
            edge_count: self.edges.len(),
            depth_range: (min_depth, max_depth),
        }
    }
}

/// A trait for critics that evaluate a `FractalField` and assign it a score.
pub trait Critic {
    /// Scores a field, typically based on its signature. Higher is usually better.
    fn score(&self, field: &FractalField) -> f32;

    /// Classifies a field into a category like "symmetric" or "chaotic".
    fn classify(&self, signature: &FractalSignature) -> String {
        if signature.is_symmetric() { "symmetric".to_string() }
        else if signature.entropy > 10.0 { "chaotic".to_string() }
        else { "structured".to_string() }
    }
}

/// A simple critic that rewards symmetry and penalizes high entropy.
pub struct SymmetryCritic;
impl Critic for SymmetryCritic {
    fn score(&self, field: &FractalField) -> f32 {
        let sig = field.signature();
        let symmetry_bonus = if sig.is_symmetric() { 1.0 } else { 0.0 };
        let entropy_penalty = sig.entropy * 0.1;
        symmetry_bonus - entropy_penalty
    }
}

/// A simple critic that rewards high entropy.
pub struct EntropyCritic;
impl Critic for EntropyCritic {
    fn score(&self, field: &FractalField) -> f32 {
        field.signature().entropy
    }
}

/// A trait for generators that produce new `FractalField` candidates.
pub trait Generator {
    /// Produces an initial set of candidates from scratch.
    fn generate(&self) -> Vec<FractalField>;
    /// Mutates an existing field to produce a new set of candidates.
    fn mutate(&self, field: &FractalField) -> Vec<FractalField>;
}

/// A trait for a single, specific mutation algorithm.
pub trait MutationStrategy {
    fn mutate(&self, field: &FractalField) -> FractalField;
}

// --- High-Level Fractal System ---

/// A helper trait to enable cloning of `Box<dyn Fractal>`.
/// This is the standard pattern for making trait objects cloneable.
pub trait FractalClone {
    fn clone_box(&self) -> Box<dyn Fractal>;
}

impl<T: 'static + Fractal + Clone> FractalClone for T {
    fn clone_box(&self) -> Box<dyn Fractal> { Box::new(self.clone()) }
}

impl Clone for Box<dyn Fractal> {
    fn clone(&self) -> Box<dyn Fractal> { self.clone_box() }
}

/// The base trait for all high-level, dynamically-typed fractal objects.
pub trait Fractal: FractalClone + Debug + Any + 'static {
    fn as_any(&self) -> &dyn Any;
    fn is_equal(&self, other: &dyn Fractal) -> bool;
    fn resonance_law(&self) -> ResonanceLaw;
    fn resonance_score(&self) -> f64;
    fn tags(&self) -> &TagSet;
    fn metadata(&self) -> &Metadata;
    fn id(&self) -> &str;
    fn children(&self) -> &[Box<dyn Fractal>];
}

/// Implements the `Fractal` trait for the base `FractalAtom` type.
impl<T> Fractal for FractalAtom<T>
where
    T: Clone + Eq + Hash + Debug + 'static + Into<f64>,
{
    fn as_any(&self) -> &dyn Any { self }

    fn is_equal(&self, other: &dyn Fractal) -> bool {
        // Check if the other trait object can be downcast to our concrete type, then compare.
        other.as_any().downcast_ref::<FractalAtom<T>>() == Some(self)
    }

    fn resonance_score(&self) -> f64 {
        // The resonance of a base atom is simply its underlying value.
        self.value.clone().into()
    }

    fn resonance_law(&self) -> ResonanceLaw {
        ResonanceLaw::Echo // A base atom is a simple echo of its value.
    }

    fn tags(&self) -> &TagSet { &self.tags }
    fn metadata(&self) -> &Metadata { &self.metadata }
    fn id(&self) -> &str { self.metadata.description.as_deref().unwrap_or("unnamed_atom") }
    fn children(&self) -> &[Box<dyn Fractal>] { &[] } // Atoms have no children.
}

/// The operation connecting a member to a `FractalCollection`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Union,        // Additive
    Difference,   // Subtractive
    Intersection, // Multiplicative
}

/// An enum that wraps all supported concrete fractal types.
///
/// This is the primary way to work with different kinds of fractals in a type-safe
/// manner. It acts as a "dispatcher", forwarding any `Fractal` trait calls to the
/// specific variant it holds.
#[derive(Debug, Clone)]
pub enum FractalType {
    Mandelbrot(Mandelbrot),
    IFS(IFS),
}

/// The `Fractal` implementation for the `FractalType` enum.
/// It delegates each method call to the active variant inside it.
impl Fractal for FractalType {
    fn as_any(&self) -> &dyn Any { self }

    fn children(&self) -> &[Box<dyn Fractal>] {
        match self {
            FractalType::Mandelbrot(m) => m.children(),
            FractalType::IFS(i) => i.children(),
        }
    }

    fn id(&self) -> &str {
        match self {
            FractalType::Mandelbrot(m) => m.id(),
            FractalType::IFS(i) => i.id(),
        }
    }

    fn is_equal(&self, other: &dyn Fractal) -> bool {
        // Downcast the `other` trait object to see if it's also a FractalType enum.
        if let Some(other_ft) = other.as_any().downcast_ref::<FractalType>() {
            // If so, compare the variants. They are only equal if they are the same variant
            // and their inner values are equal.
            match (self, other_ft) {
                (FractalType::Mandelbrot(m1), FractalType::Mandelbrot(m2)) => m1 == m2,
                (FractalType::IFS(i1), FractalType::IFS(i2)) => i1 == i2,
                _ => false, // Variants are different types (e.g., Mandelbrot vs IFS).
            }
        } else {
            false // `other` was not a FractalType enum.
        }
    }

    fn metadata(&self) -> &Metadata {
        match self {
            FractalType::Mandelbrot(m) => m.metadata(),
            FractalType::IFS(i) => i.metadata(),
        }
    }

    fn resonance_law(&self) -> ResonanceLaw {
        match self {
            FractalType::Mandelbrot(m) => m.resonance_law(),
            FractalType::IFS(i) => i.resonance_law(),
        }
    }

    fn resonance_score(&self) -> f64 {
        match self {
            FractalType::Mandelbrot(m) => m.resonance_score(),
            FractalType::IFS(i) => i.resonance_score(),
        }
    }

    fn tags(&self) -> &TagSet {
        match self {
            FractalType::Mandelbrot(m) => m.tags(),
            FractalType::IFS(i) => i.tags(),
        }
    }
}

// --- CSG (Constructive Solid Geometry) System ---

/// A member of a `FractalCollection`, pairing a fractal with an operation.
#[derive(Debug, Clone)]
pub struct CollectionMember {
    pub fractal: FractalType,
    pub operation: Operation,
}

/// Represents a complex object created by combining multiple fractals.
#[derive(Debug, Clone, Default)]
pub struct FractalCollection {
    pub members: Vec<CollectionMember>,
}

// --- Concrete Fractal Type Definitions ---

/// A concrete implementation of a Mandelbrot set fractal.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mandelbrot {
    pub center_re: f64,
    pub center_im: f64,
    pub zoom: f64,
    pub metadata: Metadata,
    pub tags: TagSet,
}

impl Fractal for Mandelbrot {
    fn as_any(&self) -> &dyn Any { self }
    fn children(&self) -> &[Box<dyn Fractal>] { &[] }
    fn id(&self) -> &str { "Mandelbrot" }
    fn is_equal(&self, other: &dyn Fractal) -> bool {
        other.as_any().downcast_ref::<Mandelbrot>() == Some(self)
    }
    fn metadata(&self) -> &Metadata { &self.metadata }
    fn tags(&self) -> &TagSet { &self.tags }
    fn resonance_score(&self) -> f64 {
        let chaos = (self.center_re.abs() + self.center_im.abs()).log(self.zoom);
        1.0 / chaos.abs() // Higher score for less chaotic regions.
    }
    fn resonance_law(&self) -> ResonanceLaw { ResonanceLaw::Echo }
}

/// A concrete implementation of an Iterated Function System fractal.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IFS {
    pub transform_count: u32,
    pub metadata: Metadata,
    pub tags: TagSet,
}

impl Fractal for IFS {
    fn as_any(&self) -> &dyn Any { self }
    fn children(&self) -> &[Box<dyn Fractal>] { &[] }
    fn id(&self) -> &str { "IFS" }
    fn is_equal(&self, other: &dyn Fractal) -> bool {
        other.as_any().downcast_ref::<IFS>() == Some(self)
    }
    fn metadata(&self) -> &Metadata { &self.metadata }
    fn tags(&self) -> &TagSet { &self.tags }
    fn resonance_score(&self) -> f64 { self.transform_count as f64 }
    fn resonance_law(&self) -> ResonanceLaw { ResonanceLaw::FractalGrowth }
}

// --- Type-Specific Operations ---

impl Mandelbrot {
    /// A specialized multiplication representing a geometric transformation.
    /// This is kept separate from general traits to avoid ambiguity and allow for
    /// type-specific logic that wouldn't fit in a generic `mul` operation.
    pub fn transform_by(&self, other: &Mandelbrot) -> Mandelbrot {
        Mandelbrot {
            center_re: self.center_re + other.center_re,
            center_im: self.center_im + other.center_im,
            zoom: self.zoom * other.zoom,
            metadata: self.metadata.clone(),
            tags: self.tags.clone(),
        }
    }
}

impl IFS {
    /// A specialized multiplication representing function composition.
    pub fn compose_with(&self, other: &IFS) -> IFS {
        IFS {
            transform_count: self.transform_count * other.transform_count,
            metadata: self.metadata.clone(),
            tags: self.tags.clone(),
        }
    }
}

// --- CSG Function Implementations ---

/// Combines two fractals into a `FractalCollection` representing their union.
pub fn add_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember { fractal: a.clone(), operation: Operation::Union },
            CollectionMember { fractal: b.clone(), operation: Operation::Union },
        ],
    }
}

/// Combines two fractals to represent `A / B`, modeled as a Union with an Intersection.
pub fn divide_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember { fractal: a.clone(), operation: Operation::Union },
            CollectionMember { fractal: b.clone(), operation: Operation::Intersection },
        ],
    }
}

/// Combines two fractals to represent `A - B`.
pub fn sub_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember { fractal: a.clone(), operation: Operation::Union },
            CollectionMember { fractal: b.clone(), operation: Operation::Difference },
        ],
    }
}

/// Combines two fractals to represent `A * B` at the CSG level (intersection).
pub fn mul_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember { fractal: a.clone(), operation: Operation::Union },
            CollectionMember { fractal: b.clone(), operation: Operation::Intersection },
        ],
    }
}

// --- Quantum Space & Semantic Traits ---

/// Represents a fractal, infinite-dimensional quantum space.
pub trait FractalQuantumSpace {
    type SemanticUnit;
    fn resonance_depth(&self) -> usize;
    fn project(&self, depth: usize) -> Vec<Self::SemanticUnit>;
    fn transform(&mut self, rule: &ResonanceRule);
    fn filter(&self, filter: &dyn ResonanceFilter) -> Vec<Self::SemanticUnit>;
    fn fractal_signature(&self) -> FractalSignature;
}

/// A trait for comparing objects based on their meaning rather than strict equality.
pub trait SemanticEq {
    fn semantic_eq(&self, other: &Self) -> bool;
}