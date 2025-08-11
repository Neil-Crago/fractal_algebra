use crate::field::FractalField;
use crate::traits::Generator;
use crate::critics::CriticSuite;
use crate::looprep::LoopReport;

pub struct GeneratorCriticLoop<G: Generator> {
    pub generator: G,
    pub critic_suite: CriticSuite,
    pub iterations: usize,
}

impl<G: Generator> GeneratorCriticLoop<G> {
    pub fn run(&self) -> Option<FractalField> {
        let mut best: Option<FractalField> = None;

        for _ in 0..self.iterations {
            let candidates = match &best {
                Some(f) => self.generator.mutate(f),
                None => self.generator.generate(),
            };

            if let Some(candidate) = self.critic_suite.select_best(&candidates) {
                if best.is_none() || self.critic_suite.score(candidate) > self.critic_suite.score(best.as_ref().unwrap()) {
                    best = Some(candidate.clone());
                }
            }
        }

        best
    }

    pub fn run_with_report(&self) -> Option<LoopReport> {
        let mut best: Option<FractalField> = None;
        let mut history = Vec::new();

        for _ in 0..self.iterations {
            let candidates = match &best {
                Some(f) => self.generator.mutate(f),
                None => self.generator.generate(),
            };

            if let Some(candidate) = self.critic_suite.select_best(&candidates) {
                let score = self.critic_suite.score(candidate);
                history.push((candidate.clone(), score));

                if best.is_none() || score > self.critic_suite.score(best.as_ref().unwrap()) {
                    best = Some(candidate.clone());
                }
            }
        }

        best.map(|f| LoopReport {
            best_field: f.clone(),
            best_score: self.critic_suite.score(&f),
            history,
        })
    }
}

/* Example usage 
let generator = RandomFieldGenerator { count: 10 };

let mut suite = CriticSuite::new();
suite.add_critic(SymmetryCritic, 0.5);
suite.add_critic(EntropyCritic, 0.5);

let loop_engine = GeneratorCriticLoop {
    generator,
    critic_suite: suite,
    iterations: 20,
};

if let Some(best_field) = loop_engine.run() {
    println!("Best field score: {}", loop_engine.critic_suite.score(&best_field));
    println!("Signature: {:?}", best_field.signature());
}
*/