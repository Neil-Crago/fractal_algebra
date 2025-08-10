//! defines the FractalField struct

use crate::edge::FractalEdge;
use crate::vec3::Vec3;
use num_complex::Complex;

#[derive(Clone, Debug)]
pub struct FractalField {
    pub edges: Vec<FractalEdge>,
}

impl FractalField {
    pub fn new(depth: u32) -> Self {
        let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let mut edges = Vec::new();

        for &dir in &[Vec3::X, Vec3::Y, Vec3::Z] {
            let edge = FractalEdge {
                origin,
                direction: dir,
                length: 1.0,
                depth: 0,
                data: Complex::new(1.0, 0.0),
            };
            edges.extend(Self::generate(edge, depth));
        }

        FractalField { edges }
    }

    fn generate(edge: FractalEdge, depth: u32) -> Vec<FractalEdge> {
        if depth == 0 {
            return vec![edge];
        }

        let mut children = vec![edge.clone()];
        let scale = 0.5 * edge.length;

        for &dir in &[Vec3::X, Vec3::Y, Vec3::Z] {
            let new_origin = edge.origin.add(edge.direction.scale(edge.length));
            let new_data = Complex::new(
                1.0 / (depth as f32 + 1.0),
                (new_origin.x + new_origin.y + new_origin.z) * 0.1,
            );

            let new_edge = FractalEdge {
                origin: new_origin,
                direction: dir,
                length: scale,
                depth: edge.depth + 1,
                data: new_data,
            };

            children.extend(Self::generate(new_edge, depth - 1));
        }

        children
    }
}