//! Defines a simple 3D vector struct (`Vec3`) and its associated mathematical operations.

use rand::Rng;
use std::ops::{Add, Mul, Neg, Rem, Sub};

/// A 3-dimensional vector with `f32` components.
/// It is `Copy`, so it can be passed by value cheaply.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    // --- Constants ---
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    // --- Methods ---

    /// Computes the dot product of two vectors.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the magnitude (or length) of the vector.
    pub fn norm(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns a new vector with the same direction but a magnitude of 1.
    /// Returns a zero vector if the magnitude is too small to avoid division by zero.
    pub fn normalize(self) -> Self {
        let mag = self.norm();
        if mag > 1e-6 {
            self * (1.0 / mag)
        } else {
            Self::ZERO
        }
    }

    /// Creates a new `Vec3` with random components in the range `[-1.0, 1.0)`.
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Vec3 {
            x: rng.random_range(-1.0..1.0),
            y: rng.random_range(-1.0..1.0),
            z: rng.random_range(-1.0..1.0),
        }
    }
}

// --- Operator Overloading ---

impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { Vec3 { x: -self.x, y: -self.y, z: -self.z } } }
impl Add for Vec3 { type Output = Self; fn add(self, other: Self) -> Self { Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z } } }
impl Sub for Vec3 { type Output = Self; fn sub(self, other: Self) -> Self { Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z } } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, scalar: f32) -> Self { Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar } } }
impl Rem<f32> for Vec3 { type Output = Self; fn rem(self, rhs: f32) -> Self { Vec3 { x: self.x % rhs, y: self.y % rhs, z: self.z % rhs } } }