use crate::field::FractalField;
use crate::traits::Critic;

pub struct CriticSuite {
    pub critics: Vec<(Box<dyn Critic>, f32)>, // (critic, weight)
}

impl Default for CriticSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl CriticSuite {
    /// Creates a new empty suite
    pub fn new() -> Self {
        CriticSuite {
            critics: Vec::new(),
        }
    }

    /// Adds a critic with a given weight
    pub fn add_critic<C: Critic + 'static>(&mut self, critic: C, weight: f32) {
        self.critics.push((Box::new(critic), weight));
    }

    /// Scores a field using all critics
    pub fn score(&self, field: &FractalField) -> f32 {
        self.critics
            .iter()
            .map(|(critic, weight)| critic.score(field) * weight)
            .sum()
    }

    /// Classifies using the highest-weighted critic
    pub fn classify(&self, field: &FractalField) -> String {
        if let Some((critic, _)) = self
            .critics
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            critic.classify(&field.signature())
        } else {
            "unclassified".to_string()
        }
    }
}

impl CriticSuite {
    pub fn select_best<'a>(&self, fields: &'a [FractalField]) -> Option<&'a FractalField> {
        fields.iter().max_by(|a, b| {
            let sa = self.score(a);
            let sb = self.score(b);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

/* Example usage

let mut suite = CriticSuite::new();
suite.add_critic(SymmetryCritic, 0.7);
suite.add_critic(EntropyCritic, 0.3);

let score = suite.score(&field);
let label = suite.classify(&field);

*/
