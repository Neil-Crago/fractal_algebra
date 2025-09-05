//! Defines `GraphEdge`, a struct representing a directed edge in a geometric space.

use crate::vec3::Vec3;
use num_complex::Complex;

/// Represents a physical, directed connection in 3D space.
///
/// It has geometric properties (`origin`, `direction`, `length`) and a `data`
/// payload holding a complex number, which can represent a physical quantity
/// like a wave's amplitude and phase.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphEdge {
    /// The spatial starting point of the edge.
    pub origin: Vec3,
    /// A normalized vector indicating the edge's direction.
    pub direction: Vec3,
    /// The scalar length of the edge.
    pub length: f32,
    /// The recursive depth or scale level of this edge in a fractal structure.
    pub depth: u32,
    /// The complex-valued data payload of the edge.
    pub data: Complex<f32>,
}

impl GraphEdge {
    /// Computes the spatial endpoint of the edge.
    /// `endpoint = origin + direction * length`
    pub fn endpoint(&self) -> Vec3 {
        self.origin + self.direction * self.length
    }

    /// Returns a new edge with its length scaled by a factor.
    pub fn scaled(&self, factor: f32) -> Self {
        GraphEdge {
            length: self.length * factor,
            ..*self // Copy the other fields using struct update syntax.
        }
    }

    /// Returns a new edge that is geometrically reversed.
    /// The new origin is the old endpoint, the direction is inverted, and the
    /// complex data is conjugated, which is typical for reversing wave-like phenomena.
    pub fn reversed(&self) -> Self {
        GraphEdge {
            origin: self.endpoint(),
            direction: -self.direction,
            data: self.data.conj(),
            ..*self
        }
    }

    /// Modulates the edge's properties based on an entropy value.
    /// This can be used to simulate noise, decay, or other environmental interactions.
    pub fn apply_entropy(&mut self, entropy: f32) {
        self.length *= 1.0 + entropy;
        self.data *= Complex::new(1.0 + entropy, entropy);
    }

    /// Computes a crude similarity score between two edges.
    /// Higher scores indicate greater similarity. The score considers direction alignment,
    /// length difference, and data difference.
    pub fn similarity(&self, other: &Self) -> f32 {
        let dir_dot = self.direction.dot(other.direction);
        let len_diff = (self.length - other.length).abs();
        let data_diff = (self.data - other.data).norm();
        // A simple linear combination for a similarity metric.
        dir_dot - len_diff - data_diff
    }
}