//! Defines functions for testing the algebraic laws of `FractalField`.
//!
//! These functions are used in unit tests to verify that `FractalField` correctly
//! adheres to mathematical axioms, such as those for a vector space.

use crate::field::FractalField;
use num_complex::Complex;

/// Tests the associativity of `FractalField` addition.
///
/// Verifies that `(a + b) + c` is equal to `a + (b + c)`.
pub fn test_associativity(a: &FractalField, b: &FractalField, c: &FractalField) -> bool {
    // Note: The `+` operator requires owned values, so we must clone.
    let left = (a.clone() + b.clone()) + c.clone();
    let right = a.clone() + (b.clone() + c.clone());
    left == right // Assumes FractalField implements PartialEq
}

/// Tests the distributivity of scalar multiplication over `FractalField` addition.
///
/// Verifies that `(a + b) * s` is equal to `(a * s) + (b * s)`.
pub fn test_distributivity(a: &FractalField, b: &FractalField, scalar: Complex<f32>) -> bool {
    let left = (a.clone() + b.clone()) * scalar;
    let right = (a.clone() * scalar) + (b.clone() * scalar);
    left == right
}