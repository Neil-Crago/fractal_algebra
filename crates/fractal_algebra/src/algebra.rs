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

    fn sub(&self, other: &Self) -> Self {
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
                    length: edge.length - matching.length,
                    depth: edge.depth,
                    data: edge.data - matching.data,
                });
            }
        }

        FractalField { edges: result }
    }
    
    /// `mul` implements the universal CSG `Intersection` operation.
    fn mul(&self, other: &Self) -> Self {
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


//////////
/*
// This allows operations like `mandelbrot + ifs`.
impl<T, U> FractalAlgebra<U> for T
where
    T: Fractal,
    U: Fractal,
{
    type Output = FractalCollection;

    /// `add` creates a collection of two `Union` members.
    fn add(self, rhs: U) -> Self::Output {
        FractalCollection {
            members: vec![
                CollectionMember { fractal: self.clone_box(), operation: Operation::Union },
                CollectionMember { fractal: rhs.clone_box(), operation: Operation::Union },
            ],
        }
    }


}

// Implementation to allow chaining operations on a `FractalCollection`.
// This enables expressions like `(A + B) * C`.
impl<T> FractalAlgebra<T> for FractalCollection
where
    T: Fractal,
{
    type Output = FractalCollection;

    fn add(mut self, rhs: T) -> Self::Output {
        self.members.push(CollectionMember { fractal: rhs.clone_box(), operation: Operation::Union });
        self
    }

    fn sub(mut self, rhs: T) -> Self::Output {
        self.members.push(CollectionMember { fractal: rhs.clone_box(), operation: Operation::Difference });
        self
    }
    
    fn mul(mut self, rhs: T) -> Self::Output {
        self.members.push(CollectionMember { fractal: rhs.clone_box(), operation: Operation::Intersection });
        self
    }
}
*/