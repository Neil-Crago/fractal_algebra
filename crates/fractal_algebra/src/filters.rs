use crate::traits::{Fractal, FractalCollection};
use crate::resonance::{ResonanceLaw, ResonanceFilter};


pub struct LawFilter {
    pub allowed: Vec<ResonanceLaw>,
}

impl ResonanceFilter for LawFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        self.allowed.contains(&fractal.resonance_law())
    }
}

pub struct ScoreFilter {
    pub min_score: f64,
}

impl ResonanceFilter for ScoreFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        fractal.resonance_score() >= self.min_score
    }
}

pub struct PredicateFilter {
    pub predicate: Box<dyn Fn(&dyn Fractal) -> bool>,
}

impl ResonanceFilter for PredicateFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        (self.predicate)(fractal)
    }
}

pub enum FilterLogic {
    And,
    Or,
    Not,
}

pub struct ComposedFilter {
    pub filters: Vec<Box<dyn ResonanceFilter>>,
    pub logic: FilterLogic,
}

impl ResonanceFilter for ComposedFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        match self.logic {
            FilterLogic::And => self.filters.iter().all(|f| f.passes(fractal)),
            FilterLogic::Or => self.filters.iter().any(|f| f.passes(fractal)),
            FilterLogic::Not => !self.filters[0].passes(fractal), // unary NOT
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterTrace {
    pub filter_name: String,
    pub passed: Vec<usize>, // indices of fractals that passed
    pub failed: Vec<usize>, // indices that failed
}


#[derive(Debug, Clone)]
pub struct ResonantFractalCollection {
    pub collection: FractalCollection,
    pub resonance_scores: Vec<f64>,
    pub resonance_laws: Vec<ResonanceLaw>, // This is now okay because ResonanceLaw is Sized
    pub average_resonance: f64,
}

impl ResonantFractalCollection {
    pub fn new(collection: FractalCollection) -> Self {
        // This implementation should now work correctly, assuming `m.fractal` is a Box<dyn Fractal>
        let resonance_scores = collection
            .members
            .iter()
            .map(|m| m.fractal.resonance_score())
            .collect::<Vec<_>>();

        let resonance_laws = collection
            .members
            .iter()
            .map(|m| m.fractal.resonance_law()) // This returns a Sized ResonanceLaw
            .collect::<Vec<_>>();

        let average_resonance = if resonance_scores.is_empty() {
            0.0
        } else {
            resonance_scores.iter().copied().sum::<f64>() / resonance_scores.len() as f64
        };

        Self {
            collection,
            resonance_scores,
            resonance_laws,
            average_resonance,
        }
    }

    /// Re-evaluates resonance after mutation or transformation.
    pub fn reevaluate(&mut self) {
        *self = Self::new(self.collection.clone());
    }
}

// Applying filters to RFC
impl ResonantFractalCollection {
    pub fn filter(&self, filter: &dyn ResonanceFilter) -> FractalCollection {
        let filtered_members = self
            .collection
            .members
            .iter()
            .filter(|m| filter.passes(&m.fractal))
            .cloned()
            .collect();

        FractalCollection {
            members: filtered_members,
        }
    }
    pub fn trace_filter(&self, filter: &dyn ResonanceFilter, name: &str) -> FilterTrace {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for (i, member) in self.collection.members.iter().enumerate() {
            if filter.passes(&member.fractal) {
                passed.push(i);
            } else {
                failed.push(i);
            }
        }

        FilterTrace {
            filter_name: name.to_string(),
            passed,
            failed,
        }
    }
}