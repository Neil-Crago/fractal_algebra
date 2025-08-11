//! defines the FractalAlgebra trait

use crate::fractaledge::FractalEdge;
use crate::graphedge::GraphEdge;
use crate::signature::FractalSignature;
use crate::field::FractalField;
use num_complex::Complex;
use num_complex::ComplexFloat;
use crate::Vec3;


pub trait FractalAlgebra {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: Complex<f32>) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn zero() -> Self;
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
            edges: self.edges.iter().zip(&other.edges).map(|(self_edge, other_edge)| {
                GraphEdge {
                    origin: self_edge.origin + other_edge.origin,
                    direction: self_edge.direction + other_edge.direction,
                    length: self_edge.length + other_edge.length,
                    depth: self_edge.depth + other_edge.depth,
                    data: self_edge.data + other_edge.data,
                }
            }).collect(),
        }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        FractalField {
        edges: self.edges.iter().map(|edge| {
            GraphEdge {
                origin: edge.origin,
                direction: edge.direction,
                length: edge.length * factor.re, // scale length by real part
                depth: edge.depth,
                data: edge.data * factor, // scale data by complex factor
            }
        }).collect(),
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        FractalField {
            edges: self.edges.iter().zip(&other.edges).map(|(self_edge, other_edge)| {
                GraphEdge {
                    origin: self_edge.origin + other_edge.origin,
                    direction: self_edge.direction + other_edge.direction,
                    length: self_edge.length * other_edge.length,
                    depth: self_edge.depth + other_edge.depth,
                    data: self_edge.data * other_edge.data,
                }
            }).collect(),
        }
    }
    
   
    fn multiply_and_preserve_symmetry(&self, other: &Self) -> Self {
        let ori = (self.edges[0].origin + other.edges[0].origin) % std::f32::consts::TAU;
        let dir = (self.edges[0].direction + other.edges[0].direction) % std::f32::consts::TAU;
        let scl = (self.edges[0].length * other.edges[0].length).abs(); // symmetry-preserving
        let dep = (self.edges[0].depth + other.edges[0].depth) as f32 % std::f32::consts::TAU;
        let dat: f32 = (self.edges[0].data * other.edges[0].data).abs(); // symmetry-preserving
        FractalField { edges: self.edges.iter().zip(&other.edges).map(|(_self_edge, _other_edge)| {
            GraphEdge {
                origin: ori,
                direction: dir,
                length: scl,
                depth: dep as u32,
                data: dat.into(),
            }
            }).collect(),
        }
    }

    fn zero() -> Self {
        FractalAlgebra::zero()
    }

    fn one() -> Self {
     FractalField { edges: vec![GraphEdge {
                origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                direction: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
                length: 1.0,
                depth: 0,
                data: Complex::new(1.0, 0.0),
            }] }
        }

    fn negate(&self) -> Self {
        FractalField {
            edges: self.edges.iter().map(|edge| {
                GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length,
                    depth: edge.depth,
                    data: -edge.data,
                }
            }).collect(),
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


