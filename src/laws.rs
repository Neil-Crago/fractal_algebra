//! defines the tests for associativity and distributivity

use crate::field::FractalField;
use crate::traits::FractalAlgebra;
use num_complex::Complex;

pub fn test_associativity(a: &FractalField, b: &FractalField, c: &FractalField) -> bool {
    let left = a.add(&b.add(c));
    let right = a.add(b).add(c);
    left.edges == right.edges
}

pub fn test_distributivity(a: &FractalField, b: &FractalField, scalar: Complex<f32>) -> bool {
    let left = a.add(b).scale(scalar);
    let right = a.scale(scalar).add(&b.scale(scalar));
    left.edges == right.edges
}