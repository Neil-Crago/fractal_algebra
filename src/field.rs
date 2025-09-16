//! Defines the `FractalField` struct, a collection of `GraphEdge`s that represents
//! a state in the simulation space.
//!
//! A `FractalField` can be thought of as a vector in a high-dimensional space,
//! where each `GraphEdge` is a basis vector. It supports fundamental vector
//! operations like addition, negation, and scalar multiplication through operator overloading.

use crate::graphedge::GraphEdge;
use crate::signature::FractalSignature;
use crate::vec3::Vec3;
use num_complex::Complex;
use rand::Rng;

/// A collection of `GraphEdge`s that represents a coherent state or pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct FractalField {
    pub edges: Vec<GraphEdge>,
}

impl FractalField {
    /// Creates a new `FractalField` with no edges (the zero vector).
    pub fn zero() -> Self {
        FractalField { edges: Vec::new() }
    }

    /// Creates a new `FractalField` with a single, default edge (the identity vector).
    pub fn one() -> Self {
        FractalField {
            edges: vec![GraphEdge {
                origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
                length: 1.0,
                depth: 0,
                data: Complex::new(1.0, 0.0),
            }],
        }
    }

    /// Creates a new `FractalField` with a single, randomized edge.
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let amp_mut = rng.random_range(0.1..10.0);
        let phase_mut = rng.random_range(0.0..std::f32::consts::TAU);

        FractalField {
            edges: vec![GraphEdge {
                origin: Vec3::random(),
                direction: Vec3::random(),
                length: rng.random_range(0.1..10.0),
                depth: rng.random_range(1..5),
                data: Complex::from_polar(amp_mut, phase_mut),
            }],
        }
    }

    /// Checks if the field is effectively zero by testing if all edge data has a negligible norm.
    pub fn is_zero(&self) -> bool {
        self.edges.iter().all(|e| e.data.norm() < 1e-6)
    }

    /// Computes a `FractalSignature` for the field.
    ///
    /// The signature is a condensed summary of the field's properties, such as total amplitude,
    /// average phase, and complexity, which can be used for classification or analysis.
    pub fn signature(&self) -> FractalSignature {
        if self.edges.is_empty() {
            return FractalSignature {
                total_amplitude: 0.0,
                average_phase: 0.0,
                entropy: 0.0,
                edge_count: 0,
                depth_range: (u32::MAX, 0),
            };
        }

        let mut total_amp = 0.0;
        let mut total_phase = 0.0;
        let mut entropy = 0.0; // A crude proxy for complexity.
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

// --- Operator Overloading ---

/// Implements the unary negation operator (`-`).
/// This creates a new field where the complex data of each edge is negated.
impl std::ops::Neg for FractalField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let edges = self
            .edges
            .into_iter()
            .map(|e| GraphEdge { data: -e.data, ..e }) // `..e` is struct update syntax
            .collect();
        FractalField { edges }
    }
}

/// Implements the multiplication operator (`*`) for scaling by a complex number.
impl std::ops::Mul<Complex<f32>> for FractalField {
    type Output = Self;

    fn mul(self, scalar: Complex<f32>) -> Self::Output {
        let edges = self
            .edges
            .into_iter()
            .map(|e| GraphEdge { data: e.data * scalar, ..e })
            .collect();
        FractalField { edges }
    }
}

/// Implements the addition operator (`+`).
/// This performs pointwise addition of the complex data of two fields.
/// It assumes that both fields have the same number and ordering of edges.
impl std::ops::Add for FractalField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let edges = self
            .edges
            .iter()
            .zip(rhs.edges.iter())
            .map(|(a, b)| GraphEdge {
                // Geometric properties are taken from `a`; only data is combined.
                origin: a.origin,
                direction: a.direction,
                length: a.length,
                depth: a.depth,
                data: a.data + b.data,
            })
            .collect();

        FractalField { edges }
    }
}