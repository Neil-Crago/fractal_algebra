use crate::vec3::Vec3;
use num_complex::Complex;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphEdge {
    pub origin: Vec3, // spatial coordinates
    pub direction: Vec3,
    pub length: f32,
    pub depth: u32,
    pub data: Complex<f32>, // optional payload
}

impl GraphEdge {
    /// Computes the endpoint of the edge
    pub fn endpoint(&self) -> Vec3 {
        self.origin + self.direction * self.length
    }

    /// Scales the edge length and direction
    pub fn scaled(&self, factor: f32) -> Self {
        GraphEdge {
            origin: self.origin,
            direction: self.direction,
            length: self.length * factor,
            depth: self.depth,
            data: self.data,
        }
    }

    /// Reverses the edge direction
    pub fn reversed(&self) -> Self {
        GraphEdge {
            origin: self.endpoint(),
            direction: -self.direction,
            length: self.length,
            depth: self.depth,
            data: self.data.conj(),
        }
    }

    /// Applies an entropy pulse to modulate length and data
    pub fn apply_entropy(&mut self, entropy: f32) {
        self.length *= 1.0 + entropy;
        self.data *= Complex::new(1.0 + entropy, entropy);
    }

    /// Computes a similarity score with another edge
    pub fn similarity(&self, other: &Self) -> f32 {
        let dir_dot = self.direction.dot(&other.direction);
        let len_diff = (self.length - other.length).abs();
        let data_diff = (self.data - other.data).norm();
        dir_dot - len_diff - data_diff
    }
}
