//! defines the FractalAlgebra trait

use crate::Vec3;
use crate::field::FractalField;
use crate::fractaledge::FractalEdge;
use crate::graphedge::GraphEdge;
use crate::signature::FractalSignature;
use num_complex::Complex;
use num_complex::ComplexFloat;

pub trait FractalAlgebra {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: Complex<f32>) -> Self;
    fn multiply(&self, other: &Self) -> Self;
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

    fn multiply(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude * other.amplitude,
            location: self.location + other.location,
            phase: self.phase + other.phase,
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
        FractalAlgebra::zero()
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
        FractalAlgebra::zero()
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


use std::fmt::Debug;

//-///////////////////////////////////////////////////////////////////////////
// 1. CORE COMPONENTS & TRAITS
//-///////////////////////////////////////////////////////////////////////////

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
pub trait Fractal: FractalClone + Debug + 'static {}

/// The operation to be applied to a member within a collection.
/// This is the core of the CSG (Constructive Solid Geometry) approach.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Union,       // Additive operation (+)
    Difference,  // Subtractive operation (-)
    Intersection, // Multiplicative operation (*)
}

/// A member of the FractalCollection, containing the fractal
/// itself and the operation that connects it to the rest of the collection.
#[derive(Debug, Clone)]
pub struct CollectionMember {
    pub fractal: Box<dyn Fractal>,
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

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    pub center_re: f64,
    pub center_im: f64,
    pub zoom: f64,
}
impl Fractal for Mandelbrot {}

#[derive(Debug, Clone)]
pub struct IFS {
    // An Iterated Function System would be a list of transformations (matrices).
    // Using a simple u32 for demonstration.
    pub transform_count: u32,
}
impl Fractal for IFS {}

//-///////////////////////////////////////////////////////////////////////////
// 3. THE FRACTAL ALGEBRA TRAIT
//-///////////////////////////////////////////////////////////////////////////



//-///////////////////////////////////////////////////////////////////////////
// 5. HYBRID MULTIPLICATION: SPECIALIZED METHODS
//-///////////////////////////////////////////////////////////////////////////

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
        }
    }

    pub fn sub(&self, other: &Mandelbrot) -> Mandelbrot {
        println!("LOG: Performing specialized geometric transformation subtraction...");
        // Placeholder logic: combines parameters in a specific way.
        Mandelbrot {
            center_re: self.center_re - other.center_re,
            center_im: self.center_im - other.center_im,
            zoom: self.zoom - other.zoom,
        }
    }

    pub fn mul(&self, other: &Mandelbrot) -> Mandelbrot {
        println!("LOG: Performing specialized geometric transformation multiplication...");
        // Placeholder logic: combines parameters in a specific way.
        Mandelbrot {
            center_re: self.center_re * other.center_re,
            center_im: self.center_im * other.center_im,
            zoom: self.zoom * other.zoom,
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
        }
    }
}

