use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::vec3::Vec3;
use num_complex::Complex;

pub fn canonical_test_fractal() -> FractalField {
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut edges = Vec::new();

    let directions = [Vec3::X, Vec3::Y, Vec3::Z];
    let values = [
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 1.0),
        Complex::new(1.0, 1.0),
    ];

    for (dir, val) in directions.iter().zip(values.iter()) {
        edges.push(GraphEdge {
            origin,
            direction: *dir,
            length: 1.0,
            depth: 0,
            data: *val,
        });
    }

    FractalField { edges }
}
