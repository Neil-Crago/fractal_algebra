//! Provides shared utilities and data fixtures for use in tests.

use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::vec3::Vec3;
use num_complex::Complex;

/// Creates a canonical, non-trivial `FractalField` for consistent testing.
///
/// This function produces the same field every time it's called, containing
/// three edges along the X, Y, and Z axes. This is useful for testing algebraic
/// properties where you need a predictable starting point.
pub fn canonical_test_fractal() -> FractalField {
    let origin = Vec3::ZERO;
    let edges = vec![
        GraphEdge {
            origin,
            direction: Vec3::X,
            length: 1.0,
            depth: 0,
            data: Complex::new(1.0, 0.0),
        },
        GraphEdge {
            origin,
            direction: Vec3::Y,
            length: 1.0,
            depth: 0,
            data: Complex::new(0.0, 1.0),
        },
        GraphEdge {
            origin,
            direction: Vec3::Z,
            length: 1.0,
            depth: 0,
            data: Complex::new(1.0, 1.0),
        },
    ];
    FractalField { edges }
}