//! defines the various field structs

use crate::graphedge::GraphEdge;
use crate::signature::FractalSignature;
use crate::vec3::Vec3;
use num_complex::Complex;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct FractalField {
    pub edges: Vec<GraphEdge>,
}

impl FractalField {
    /// Returns the zero field (no edges)
    pub fn zero() -> Self {
        FractalField { edges: Vec::new() }
    }

    /// Returns the identity field (single unit edge)
    pub fn one() -> Self {
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

    /// Returns the additive inverse of the field
    pub fn negate(&self) -> Self {
        let edges = self
            .edges
            .iter()
            .map(|e| GraphEdge {
                origin: e.origin,
                direction: e.direction,
                length: e.length,
                depth: e.depth,
                data: -e.data,
            })
            .collect();

        FractalField { edges }
    }

    /// Checks if all edge data is zero
    pub fn is_zero(&self) -> bool {
        self.edges.iter().all(|e| e.data.norm() < 1e-6)
    }

    /// Scales all edge data by a complex scalar
    pub fn scale(&self, scalar: Complex<f32>) -> Self {
        let edges = self
            .edges
            .iter()
            .map(|e| GraphEdge {
                origin: e.origin,
                direction: e.direction,
                length: e.length,
                depth: e.depth,
                data: e.data * scalar,
            })
            .collect();

        FractalField { edges }
    }

    /// Adds two fields pointwise (assumes matching structure)
    pub fn add(&self, other: &Self) -> Self {
        let edges = self
            .edges
            .iter()
            .zip(&other.edges)
            .map(|(a, b)| GraphEdge {
                origin: a.origin,
                direction: a.direction,
                length: a.length,
                depth: a.depth,
                data: a.data + b.data,
            })
            .collect();

        FractalField { edges }
    }

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
}

impl FractalField {
    pub fn signature(&self) -> FractalSignature {
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
            entropy += amp * phase.abs(); // crude entropy proxy

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

impl std::ops::Neg for FractalField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let edges = self
            .edges
            .into_iter()
            .map(|e| GraphEdge { data: -e.data, ..e })
            .collect();

        FractalField { edges }
    }
}

impl std::ops::Mul<Complex<f32>> for FractalField {
    type Output = Self;

    fn mul(self, scalar: Complex<f32>) -> Self::Output {
        let edges = self
            .edges
            .into_iter()
            .map(|e| GraphEdge {
                data: e.data * scalar,
                ..e
            })
            .collect();

        FractalField { edges }
    }
}

impl std::ops::Add for FractalField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let edges = self
            .edges
            .iter()
            .zip(rhs.edges.iter())
            .map(|(a, b)| GraphEdge {
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
