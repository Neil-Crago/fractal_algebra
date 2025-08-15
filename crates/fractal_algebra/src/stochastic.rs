use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::traits::MutationStrategy;
use num_complex::Complex;
use rand::Rng;

pub struct StochasticAmplitudePhase {
    pub amplitude_jitter: f32,
    pub phase_jitter: f32,
}

impl MutationStrategy for StochasticAmplitudePhase {
    fn mutate(&self, field: &FractalField) -> FractalField {
        let mut rng = rand::rng();

        let edges = field
            .edges
            .iter()
            .map(|e| {
                let amp = e.data.norm();
                let phase = e.data.arg();

                let amp_mut = amp + rng.random_range(-self.amplitude_jitter..self.amplitude_jitter);
                let phase_mut = phase + rng.random_range(-self.phase_jitter..self.phase_jitter);

                GraphEdge {
                    origin: e.origin,
                    direction: e.direction,
                    length: e.length,
                    depth: e.depth,
                    data: Complex::from_polar(amp_mut, phase_mut),
                }
            })
            .collect();

        FractalField { edges }
    }
}
