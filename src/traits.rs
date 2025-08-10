//! defines the FractalAlgebra trait

use num_complex::Complex;

pub trait FractalAlgebra {
    fn add(&self, other: &Self) -> Self;
    fn scale(&self, factor: Complex<f32>) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn zero() -> Self;
}

