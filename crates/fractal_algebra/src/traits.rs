//! defines the FractalAlgebra trait

use crate::Vec3;
use crate::atom::{FractalAtom, Metadata, TagSet};
use crate::field::FractalField;
use crate::fractaledge::FractalEdge;
use crate::graphedge::GraphEdge;
use crate::resonance::{ResonanceFilter, ResonanceLaw, ResonanceRule};
use crate::signature::FractalSignature;
use num_complex::Complex;
use num_complex::ComplexFloat;
use std::any::Any;
use std::f64::consts::PI;
use std::fmt::Debug;
use std::hash::Hash;

pub trait FractalAlgebra {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: Complex<f32>) -> Self;
    fn zero() -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
}

pub trait FractalRing: Sized {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: Complex<f32>) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn multiply_and_preserve_symmetry(&self, other: &Self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn negate(&self) -> Self;
}

impl FractalAlgebra for FractalEdge {
    fn add(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude + other.amplitude,
            location: self.location, // or average?
            phase: self.phase + other.phase,
        }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        FractalEdge {
            amplitude: self.amplitude * factor,
            location: self.location,
            phase: self.phase + factor.arg(),
        }
    }

    fn zero() -> Self {
        FractalEdge {
            amplitude: Complex::new(0.0, 0.0),
            location: 0,
            phase: 0.0,
        }
    }

    /// `sub` implements the `A + (-B)` model.
    /// It creates a collection with a `Union` (A) and a `Difference` (-B).
    fn sub(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude - other.amplitude,
            location: self.location,
            phase: self.phase - other.phase,
        }
    }

    /// `mul` implements the universal CSG `Intersection` operation.
    fn mul(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude * other.amplitude,
            location: self.location + other.location,
            phase: self.phase + other.phase,
        }
    }
}

impl FractalRing for FractalEdge {
    fn add(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude + other.amplitude,
            location: self.location, // or average?
            phase: self.phase + other.phase,
        }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        FractalEdge {
            amplitude: self.amplitude * factor,
            location: self.location,
            phase: self.phase + factor.arg(),
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude * other.amplitude,
            location: self.location + other.location,
            phase: self.phase + other.phase,
        }
    }

    fn multiply_and_preserve_symmetry(&self, other: &Self) -> Self {
        let amp = (self.amplitude * other.amplitude).abs(); // symmetry-preserving
        let phase = (self.phase + other.phase) % std::f32::consts::TAU;
        FractalEdge {
            amplitude: amp.into(),
            location: self.location + other.location,
            phase,
        }
    }

    fn zero() -> Self {
        FractalEdge {
            amplitude: Complex::new(0.0, 0.0),
            location: 0,
            phase: 0.0,
        }
    }

    fn one() -> Self {
        FractalEdge {
            amplitude: 1.0.into(),
            location: 0,
            phase: 0.0,
        }
    }

    fn negate(&self) -> Self {
        FractalEdge {
            amplitude: -self.amplitude,
            location: self.location,
            phase: -self.phase,
        }
    }
}

trait _AlgebraicLaws {
    fn test_commutativity(&self) -> bool;
    fn test_associativity(&self) -> bool;
}

impl FractalRing for FractalField {
    fn add(&self, other: &Self) -> Self {
        FractalField {
            edges: self
                .edges
                .iter()
                .zip(&other.edges)
                .map(|(self_edge, other_edge)| GraphEdge {
                    origin: self_edge.origin + other_edge.origin,
                    direction: self_edge.direction + other_edge.direction,
                    length: self_edge.length + other_edge.length,
                    depth: self_edge.depth + other_edge.depth,
                    data: self_edge.data + other_edge.data,
                })
                .collect(),
        }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        FractalField {
            edges: self
                .edges
                .iter()
                .map(|edge| {
                    GraphEdge {
                        origin: edge.origin,
                        direction: edge.direction,
                        length: edge.length * factor.re, // scale length by real part
                        depth: edge.depth,
                        data: edge.data * factor, // scale data by complex factor
                    }
                })
                .collect(),
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        FractalField {
            edges: self
                .edges
                .iter()
                .zip(&other.edges)
                .map(|(self_edge, other_edge)| GraphEdge {
                    origin: self_edge.origin + other_edge.origin,
                    direction: self_edge.direction + other_edge.direction,
                    length: self_edge.length * other_edge.length,
                    depth: self_edge.depth + other_edge.depth,
                    data: self_edge.data * other_edge.data,
                })
                .collect(),
        }
    }

    fn multiply_and_preserve_symmetry(&self, other: &Self) -> Self {
        let ori = (self.edges[0].origin + other.edges[0].origin) % std::f32::consts::TAU;
        let dir = (self.edges[0].direction + other.edges[0].direction) % std::f32::consts::TAU;
        let scl = (self.edges[0].length * other.edges[0].length).abs(); // symmetry-preserving
        let dep = (self.edges[0].depth + other.edges[0].depth) as f32 % std::f32::consts::TAU;
        let dat: f32 = (self.edges[0].data * other.edges[0].data).abs(); // symmetry-preserving
        FractalField {
            edges: self
                .edges
                .iter()
                .zip(&other.edges)
                .map(|(_self_edge, _other_edge)| GraphEdge {
                    origin: ori,
                    direction: dir,
                    length: scl,
                    depth: dep as u32,
                    data: dat.into(),
                })
                .collect(),
        }
    }

    fn zero() -> Self {
        FractalField { edges: Vec::new() }
    }

    fn one() -> Self {
        FractalField {
            edges: vec![GraphEdge {
                origin: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                direction: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                length: 1.0,
                depth: 0,
                data: Complex::new(1.0, 0.0),
            }],
        }
    }

    fn negate(&self) -> Self {
        FractalField {
            edges: self
                .edges
                .iter()
                .map(|edge| GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length,
                    depth: edge.depth,
                    data: -edge.data,
                })
                .collect(),
        }
    }
}

pub trait HasSignature {
    fn signature(&self) -> FractalSignature;
}

impl HasSignature for FractalField {
    fn signature(&self) -> FractalSignature {
        let mut total_amp = 0.0;
        let mut total_phase = 0.0;
        let mut entropy = 0.0;
        let mut min_depth = u32::MAX;
        let mut max_depth = 0;

        for edge in &self.edges {
            let amp = edge.data.norm();
            let phase = edge.data.arg();

            total_amp += amp;
            total_phase += phase;
            entropy += amp * phase.abs();

            min_depth = min_depth.min(edge.depth);
            max_depth = max_depth.max(edge.depth);
        }

        let count = self.edges.len();
        let avg_phase = if count > 0 {
            total_phase / count as f32
        } else {
            0.0
        };

        FractalSignature {
            total_amplitude: total_amp,
            average_phase: avg_phase,
            entropy,
            edge_count: count,
            depth_range: (min_depth, max_depth),
        }
    }
}

pub trait Critic {
    /// Scores a field based on its signature
    fn score(&self, field: &FractalField) -> f32;

    /// Optionally classifies the field (e.g. "symmetric", "chaotic")
    fn classify(&self, signature: &FractalSignature) -> String {
        if signature.is_symmetric() {
            "symmetric".to_string()
        } else if signature.entropy > 10.0 {
            "chaotic".to_string()
        } else {
            "structured".to_string()
        }
    }
}

pub struct SymmetryCritic;

impl Critic for SymmetryCritic {
    fn score(&self, field: &FractalField) -> f32 {
        let sig = field.signature();
        let symmetry_bonus = if sig.is_symmetric() { 1.0 } else { 0.0 };
        let entropy_penalty = sig.entropy * 0.1;
        symmetry_bonus - entropy_penalty
    }
}

pub struct EntropyCritic;

impl Critic for EntropyCritic {
    fn score(&self, field: &FractalField) -> f32 {
        let sig = field.signature();
        sig.entropy
    }
}

pub trait Generator {
    /// Produces initial candidates
    fn generate(&self) -> Vec<FractalField>;

    /// Mutates a field to produce variations
    fn mutate(&self, field: &FractalField) -> Vec<FractalField>;
}

pub trait MutationStrategy {
    /// Mutates a field to produce a new variant
    fn mutate(&self, field: &FractalField) -> FractalField;
}

/// A trait to allow cloning of Box<dyn Fractal>.
/// This is a standard pattern for cloning trait objects in Rust.
pub trait FractalClone {
    fn clone_box(&self) -> Box<dyn Fractal>;
}

impl<T> FractalClone for T
where
    T: 'static + Fractal + Clone,
{
    fn clone_box(&self) -> Box<dyn Fractal> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Fractal> {
    fn clone(&self) -> Box<dyn Fractal> {
        self.clone_box()
    }
}

/// The base trait for all fractal types.
/// It requires Clone, Debug, and a static lifetime.
/// Only object-safe methods are allowed for dyn compatibility.
pub trait Fractal: FractalClone + Debug + Any + 'static {
    /// Returns a reference to the fractal as a trait object.
    fn as_any(&self) -> &dyn Any;

    /// Checks if two fractals are equal.
    fn is_equal(&self, other: &dyn Fractal) -> bool;

    /// Returns the resonance law governing this fractal.
    fn resonance_law(&self) -> ResonanceLaw;

    /// Returns the resonance score of this fractal.
    fn resonance_score(&self) -> f64;

    /// Returns the semantic tags associated with this fractal.
    fn tags(&self) -> &TagSet;

    /// Returns the metadata describing this fractal.
    fn metadata(&self) -> &Metadata;

    /// Optional: Returns a unique identifier or label.
    fn id(&self) -> &str;

    /// Optional: Returns the fractal’s children or substructure.
    fn children(&self) -> &[Box<dyn Fractal>];
}

impl<T> Fractal for FractalAtom<T>
where
    T: Clone + Eq + Hash + Debug + 'static + Into<f64>,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_equal(&self, other: &dyn Fractal) -> bool {
        other
            .as_any()
            .downcast_ref::<FractalAtom<T>>()
            .map_or(false, |o| self == o)
    }

    fn resonance_score(&self) -> f64 {
        self.value.clone().into()
    }

    fn resonance_law(&self) -> ResonanceLaw {
        ResonanceLaw::Echo
    }

    fn tags(&self) -> &TagSet {
        &self.tags
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn id(&self) -> &str {
        self.metadata.description.as_deref().unwrap_or("unnamed")
    }

    fn children(&self) -> &[Box<dyn Fractal>] {
        &[] // Atoms are leaf nodes by default
    }
}

/// The operation to be applied to a member within a collection.
/// This is the core of the CSG (Constructive Solid Geometry) approach.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Union,        // Additive operation (+)
    Difference,   // Subtractive operation (-)
    Intersection, // Multiplicative operation (*)
}
/// Enum to wrap all supported fractal types.
#[derive(Debug, Clone)]
pub enum FractalType {
    Mandelbrot(Mandelbrot),
    IFS(IFS),
    // Add more variants as needed
}

impl Fractal for FractalType {
    fn as_any(&self) -> &dyn Any {
        self
    }

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
        match (self, other.as_any().downcast_ref::<FractalType>()) {
            (FractalType::Mandelbrot(m1), Some(FractalType::Mandelbrot(m2))) => m1 == m2,
            (FractalType::IFS(i1), Some(FractalType::IFS(i2))) => i1 == i2,
            _ => false,
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

/// A member of the FractalCollection, containing the fractal
/// itself and the operation that connects it to the rest of the collection.
#[derive(Debug, Clone)]
pub struct CollectionMember {
    pub fractal: FractalType,
    pub operation: Operation,
}

/// Represents a collection of fractals and their relationships.
/// This is the primary output of most algebraic operations.
#[derive(Debug, Clone, Default)]
pub struct FractalCollection {
    pub members: Vec<CollectionMember>,
}

//-///////////////////////////////////////////////////////////////////////////
// 2. CONCRETE FRACTAL TYPE DEFINITIONS (EXAMPLES)
//-///////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mandelbrot {
    pub center_re: f64,
    pub center_im: f64,
    pub zoom: f64,
    pub metadata: Metadata,
    pub tags: TagSet,
}

impl Fractal for Mandelbrot {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn children(&self) -> &[Box<dyn Fractal>] {
        &[]
    }

    fn id(&self) -> &str {
        "Mandelbrot"
    }

    fn is_equal(&self, other: &dyn Fractal) -> bool {
        match other.as_any().downcast_ref::<Mandelbrot>() {
            Some(o) => self == o,
            None => false,
        }
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn tags(&self) -> &TagSet {
        &self.tags
    }

    fn resonance_score(&self) -> f64 {
        // Score = amplitude magnitude × phase alignment factor
        let amp = self.center_re.abs() + self.center_im.abs() + self.zoom.abs();
        let phase_alignment = (1.0 - (self.center_re % (2.0 * PI)).cos()).abs();
        amp * phase_alignment
    }
    fn resonance_law(&self) -> ResonanceLaw {
        ResonanceLaw::Echo
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IFS {
    pub transform_count: u32,
    pub metadata: Metadata,
    pub tags: TagSet,
}
impl Fractal for IFS {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn children(&self) -> &[Box<dyn Fractal>] {
        &[]
    }

    fn id(&self) -> &str {
        "IFS"
    }

    fn is_equal(&self, other: &dyn Fractal) -> bool {
        match other.as_any().downcast_ref::<IFS>() {
            Some(o) => self == o,
            None => false,
        }
    }

    fn metadata(&self) -> &Metadata {
    &self.metadata
}

fn tags(&self) -> &TagSet {
    &self.tags
}

    fn resonance_score(&self) -> f64 {
        // Score = transform count × average transformation complexity
        self.transform_count as f64 * 1.0 // Placeholder for complexity
    }
    fn resonance_law(&self) -> ResonanceLaw {
        ResonanceLaw::FractalGrowth
    }
}

impl Mandelbrot {
    /// Specialized Multiplication: Geometric Transformation.
    /// This is not part of the `FractalAlgebra` trait to avoid ambiguity.
    pub fn transform_by(&self, other: &Mandelbrot) -> Mandelbrot {
        println!("LOG: Performing specialized geometric transformation multiplication...");
        // Placeholder logic: combines parameters in a specific way.
        Mandelbrot {
            center_re: self.center_re + other.center_re,
            center_im: self.center_im + other.center_im,
            zoom: self.zoom * other.zoom,
            metadata: self.metadata.clone(), // or merge metadata?
            tags: self.tags.clone(), // or merge tags?
        }
    }

    pub fn sub(&self, other: &Mandelbrot) -> Mandelbrot {
        println!("LOG: Performing specialized geometric transformation subtraction...");
        // Placeholder logic: combines parameters in a specific way.
        Mandelbrot {
            center_re: self.center_re - other.center_re,
            center_im: self.center_im - other.center_im,
            zoom: self.zoom - other.zoom,
            metadata: self.metadata.clone(), // or merge metadata?
            tags: self.tags.clone(), // or merge tags?
        }
    }

    pub fn mul(&self, other: &Mandelbrot) -> Mandelbrot {
        println!("LOG: Performing specialized geometric transformation multiplication...");
        // Placeholder logic: combines parameters in a specific way.
        Mandelbrot {
            center_re: self.center_re * other.center_re,
            center_im: self.center_im * other.center_im,
            zoom: self.zoom * other.zoom,
            metadata: self.metadata.clone(), // or merge metadata?
            tags: self.tags.clone(), // or merge tags?
        }
    }
}

impl IFS {
    /// Specialized Multiplication: Function Composition.
    /// This is not part of the `FractalAlgebra` trait to avoid ambiguity.
    pub fn compose_with(&self, other: &IFS) -> IFS {
        println!("LOG: Performing specialized function composition multiplication...");
        // Placeholder logic: The new IFS would have n * m transforms.
        IFS {
            transform_count: self.transform_count * other.transform_count,
            metadata: self.metadata.clone(), // or merge metadata?
            tags: self.tags.clone(), // or merge tags?
        }
    }
}

impl FractalCollection {
    pub fn resonance_scores(&self) -> Vec<f64> {
        self.members
            .iter()
            .map(|member| match &member.fractal {
                FractalType::Mandelbrot(m) => <Mandelbrot as Fractal>::resonance_score(m),
                FractalType::IFS(i) => <IFS as Fractal>::resonance_score(i),
            })
            .collect()
    }

    pub fn resonance_laws(&self) -> Vec<ResonanceLaw> {
        self.members
            .iter()
            .map(|member| match &member.fractal {
                FractalType::Mandelbrot(m) => <Mandelbrot as Fractal>::resonance_law(m),
                FractalType::IFS(i) => <IFS as Fractal>::resonance_law(i),
            })
            .collect()
    }

    fn _average_resonance(&self) -> f64 {
        let scores = self.resonance_scores();
        if scores.is_empty() {
            0.0
        } else {
            scores.iter().copied().sum::<f64>() / scores.len() as f64
        }
    }
}

/// A trait defining algebraic operations for fractal fields.
impl FractalAlgebra for FractalField {
    fn add(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for edge in &self.edges {
            if let Some(matching) = other
                .edges
                .iter()
                .find(|e2| e2.origin == edge.origin && e2.direction == edge.direction)
            {
                result.push(GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length,
                    depth: edge.depth,
                    data: edge.data + matching.data,
                });
            } else {
                result.push(edge.clone());
            }
        }

        FractalField { edges: result }
    }

    fn zero() -> Self {
        FractalField { edges: Vec::new() }
    }

    fn sub(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for edge in &self.edges {
            if let Some(matching) = other
                .edges
                .iter()
                .find(|e2| e2.origin == edge.origin && e2.direction == edge.direction)
            {
                result.push(GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length - matching.length,
                    depth: edge.depth,
                    data: edge.data - matching.data,
                });
            }
        }

        FractalField { edges: result }
    }

    /// `mul` implements the universal CSG `Intersection` operation.
    fn mul(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for edge in &self.edges {
            if let Some(matching) = other
                .edges
                .iter()
                .find(|e2| e2.origin == edge.origin && e2.direction == edge.direction)
            {
                result.push(GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length * matching.length,
                    depth: edge.depth,
                    data: edge.data * matching.data,
                });
            }
        }

        FractalField { edges: result }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        let edges = self
            .edges
            .iter()
            .map(|e| GraphEdge {
                data: e.data * factor,
                ..*e
            })
            .collect();

        FractalField { edges }
    }
}

// This allows operations like `mandelbrot + ifs`.
pub fn add_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember {
                fractal: a.clone(),
                operation: Operation::Union,
            },
            CollectionMember {
                fractal: b.clone(),
                operation: Operation::Union,
            },
        ],
    }
}

pub fn sub_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember {
                fractal: a.clone(),
                operation: Operation::Union,
            },
            CollectionMember {
                fractal: b.clone(),
                operation: Operation::Difference,
            },
        ],
    }
}

pub fn mul_fractals(a: &FractalType, b: &FractalType) -> FractalCollection {
    FractalCollection {
        members: vec![
            CollectionMember {
                fractal: a.clone(),
                operation: Operation::Union,
            },
            CollectionMember {
                fractal: b.clone(),
                operation: Operation::Intersection,
            },
        ],
    }
}

// Implementation to allow chaining operations on a `FractalCollection`.
// This enables expressions like `(A + B) * C`.
impl FractalAlgebra for FractalCollection {
    fn add(&self, other: &Self) -> Self {
        let mut members = self.members.clone();
        members.extend(other.members.clone());
        FractalCollection { members }
    }

    fn sub(&self, other: &Self) -> Self {
        let mut members = self.members.clone();
        for member in &other.members {
            members.push(CollectionMember {
                fractal: member.fractal.clone(),
                operation: Operation::Difference,
            });
        }
        FractalCollection { members }
    }

    fn zero() -> Self {
        FractalCollection {
            members: Vec::new(),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut members = self.members.clone();
        for member in &other.members {
            members.push(CollectionMember {
                fractal: member.fractal.clone(),
                operation: Operation::Intersection,
            });
        }
        FractalCollection { members }
    }

    fn scale(&self, _factor: Complex<f32>) -> Self {
        // Scaling not implemented for FractalCollection
        self.clone()
    }
}

/// Represents a fractal, infinite-dimensional quantum space
/// where each dimension corresponds to a semantic axis of resonance.
/// Transformations reveal deeper structure recursively.
pub trait FractalQuantumSpace {
    /// The base semantic unit—could be a wavefunction, resonance node, or law fragment.
    type SemanticUnit;

    /// Returns the current resonance depth (how many recursive layers have been revealed).
    fn resonance_depth(&self) -> usize;

    /// Projects the space onto a finite slice for observation or measurement.
    fn project(&self, depth: usize) -> Vec<Self::SemanticUnit>;

    /// Applies a transformation that recursively deepens the semantic structure.
    fn transform(&mut self, rule: &ResonanceRule);

    /// Filters semantic units based on resonance criteria.
    fn filter(&self, filter: &dyn ResonanceFilter) -> Vec<Self::SemanticUnit>;

    /// Returns a fractal signature—a symbolic representation of the space's current structure.
    fn fractal_signature(&self) -> FractalSignature;
}

pub trait SemanticEq {
    fn semantic_eq(&self, other: &Self) -> bool;
}
