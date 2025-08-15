use num_complex::Complex32; // alias for Complex<f32>
pub type Scalar = Complex32;
use crate::constants::MODULUS;

#[derive(Clone, Debug, PartialEq)]
pub struct FractalEdge {
    pub amplitude: Scalar, // signal strength
    pub location: usize,   // modular or codon index
    pub phase: f32,        // radians or modular angle
}

impl FractalEdge {
    pub fn convolve(&self, other: &Self) -> Self {
        FractalEdge {
            amplitude: self.amplitude * other.amplitude,
            location: (self.location + other.location) % MODULUS,
            phase: self.phase + other.phase, // consider wrapping
        }
    }

    pub fn combine(&self, other: &Self) -> Self {
        assert!((self.phase - other.phase).abs() < 1e-3, "Phases must match");

        FractalEdge {
            amplitude: self.amplitude + other.amplitude,
            location: self.location,
            phase: self.phase,
        }
    }
}
