//! defines the FractalAlgebra trait

use crate::fractaledge::FractalEdge;
use num_complex::Complex;
use num_complex::ComplexFloat;

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
