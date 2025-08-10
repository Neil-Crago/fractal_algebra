//! defines the FractalEdge struct

use crate::vec3::Vec3;
use num_complex::Complex;

#[derive(Clone, Debug, PartialEq)]
pub struct FractalEdge {
    pub origin: Vec3,
    pub direction: Vec3,
    pub length: f32,
    pub depth: u32,
    pub data: Complex<f32>,
}