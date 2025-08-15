use crate::field::FractalField;
use crate::graphedge::GraphEdge;
use crate::traits::Generator;
use crate::vec3::Vec3;
use num_complex::Complex;
use rand::Rng;

pub struct RandomFieldGenerator {
    pub count: usize,
    pub mutation_strength: f32,
}

impl Generator for RandomFieldGenerator {
    fn generate(&self) -> Vec<FractalField> {
        let mut rng = rand::rng();
        (0..self.count)
            .map(|_| {
                let edges = (0..5)
                    .map(|_| GraphEdge {
                        origin: Vec3::random(),
                        direction: Vec3::unit_x(),
                        length: 1.0,
                        depth: rng.random_range(0..5),
                        data: Complex::new(rng.random(), rng.random()),
                    })
                    .collect();

                FractalField { edges }
            })
            .collect()
    }

    fn mutate(&self, field: &FractalField) -> Vec<FractalField> {
        let mut rng = rand::rng();

        (0..self.count)
            .map(|_| {
                let edges = field
                    .edges
                    .iter()
                    .map(|e| {
                        let amp = e.data.norm();
                        let phase = e.data.arg();

                        let amp_mut =
                            amp + rng.random_range(-self.mutation_strength..self.mutation_strength);
                        let phase_mut = phase
                            + rng.random_range(-self.mutation_strength..self.mutation_strength);

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
            })
            .collect()
    }
}

/* Example usage

let generator = RandomFieldGenerator { count: 10 };
let candidates = generator.generate();

let mut suite = CriticSuite::new();
suite.add_critic(SymmetryCritic, 0.6);
suite.add_critic(EntropyCritic, 0.4);

if let Some(best) = suite.select_best(&candidates) {
    println!("Best field score: {}", suite.score(best));
    println!("Classification: {}", suite.classify(best));
}

*/
