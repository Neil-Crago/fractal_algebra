//! A module defining algebraic operations for fractal structures.

use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::traits::FractalAlgebra;
use num_complex::Complex;

/// A trait defining algebraic operations for fractal fields.
impl FractalAlgebra for FractalField {
    fn add(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for edge in &self.edges {
            if let Some(matching) = other
                .edges
                .iter()
                .find(|e2| e2.origin == edge.origin && e2.direction == edge.direction)
            {
                result.push(GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length,
                    depth: edge.depth,
                    data: edge.data + matching.data,
                });
            } else {
                result.push(edge.clone());
            }
        }

        FractalField { edges: result }
    }

    fn scale(&self, factor: Complex<f32>) -> Self {
        let edges = self
            .edges
            .iter()
            .map(|e| GraphEdge {
                data: e.data * factor,
                ..*e
            })
            .collect();

        FractalField { edges }
    }

    fn multiply(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for edge in &self.edges {
            if let Some(matching) = other
                .edges
                .iter()
                .find(|e2| e2.origin == edge.origin && e2.direction == edge.direction)
            {
                result.push(GraphEdge {
                    origin: edge.origin,
                    direction: edge.direction,
                    length: edge.length * matching.length,
                    depth: edge.depth,
                    data: edge.data * matching.data,
                });
            }
        }

        FractalField { edges: result }
    }

    fn zero() -> Self {
        FractalField { edges: Vec::new() }
    }
}
