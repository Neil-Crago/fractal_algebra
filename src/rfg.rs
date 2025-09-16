//! Defines a `Generator` that produces `FractalField`s with random properties.

use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::traits::Generator;
use crate::vec3::Vec3;
use num_complex::Complex;
use rand::Rng;

/// A generator that creates a population of `FractalField`s with random edge data.
///
/// # Examples
///
/// ```no_run
/// use fractal_algebra::{RandomFieldGenerator, Generator, CriticSuite, SymmetryCritic};
///
/// let generator = RandomFieldGenerator { count: 10, mutation_strength: 0.5 };
/// let initial_candidates = generator.generate();
///
/// let mut suite = CriticSuite::new();
/// suite.add_critic(SymmetryCritic, 1.0);
///
/// if let Some(best) = suite.select_best(&initial_candidates) {
///     println!("Best initial field score: {}", suite.score(best));
/// }
/// ```
pub struct RandomFieldGenerator {
    /// The number of candidates to produce in each `generate` or `mutate` call.
    pub count: usize,
    /// The magnitude of the random changes to apply during mutation.
    pub mutation_strength: f32,
}

impl Generator for RandomFieldGenerator {
    /// Produces an initial population of `FractalField`s, each with random edges.
    fn generate(&self) -> Vec<FractalField> {
        let mut rng = rand::rng();
        (0..self.count)
            .map(|_| {
                let edges = (0..5) // Generate 5 random edges per field
                    .map(|_| GraphEdge {
                        origin: Vec3::random(),
                        direction: Vec3::X,
                        length: 1.0,
                        depth: rng.random_range(0..5),
                        data: Complex::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)),
                    })
                    .collect();
                FractalField { edges }
            })
            .collect()
    }

    /// Mutates a given field by creating `count` new variations, each with stochastically
    /// altered amplitude and phase for every edge.
    fn mutate(&self, field: &FractalField) -> Vec<FractalField> {
        let mut rng = rand::rng();

        (0..self.count)
            .map(|_| {
                let edges = field
                    .edges
                    .iter()
                    .map(|e| {
                        let (amp, phase) = e.data.to_polar();
                        let amp_mut = amp + rng.random_range(-self.mutation_strength..self.mutation_strength);
                        let phase_mut = phase + rng.random_range(-self.mutation_strength..self.mutation_strength);

                        GraphEdge {
                            data: Complex::from_polar(amp_mut.max(0.0), phase_mut), // Ensure amplitude is non-negative
                            ..*e
                        }
                    })
                    .collect();
                FractalField { edges }
            })
            .collect()
    }
}