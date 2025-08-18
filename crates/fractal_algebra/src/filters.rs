use crate::traits::{Fractal, FractalCollection};
use crate::resonance::{ResonanceLaw, ResonanceFilter, SemanticUnit};

pub struct LawFilter {
    pub allowed: Vec<ResonanceLaw>,
}

impl ResonanceFilter for LawFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        self.allowed.contains(&fractal.resonance_law())
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units.iter()
            // Example: filter by label instead of law
            .filter(|u| self.allowed.iter().any(|law| u.label == law.to_string()))
            .cloned()
            .collect()
    }
}

pub struct ScoreFilter {
    pub min_score: f64,
}

impl ResonanceFilter for ScoreFilter {
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        fractal.resonance_score() >= self.min_score
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        // No 'score' field available, so filter by 'phase' as a proxy for score
        units.iter()
            .filter(|u| u.phase >= self.min_score)
            .cloned()
            .collect()
    }
}

pub struct PredicateFilter {
    pub predicate: Box<dyn Fn(&SemanticUnit) -> bool>,
}

impl ResonanceFilter for PredicateFilter {
    fn passes(&self, _fractal: &dyn Fractal) -> bool {
        // Not possible to use the predicate directly on Fractal, so always return true or false.
        // You may want to implement this differently depending on your use case.
        true
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units.iter()
            .filter(|u| (self.predicate)(u))
            .cloned()
            .collect()
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

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        match self.logic {
            FilterLogic::And => {
                self.filters.iter().fold(units.to_vec(), |acc, f| f.apply(&acc))
            }
            FilterLogic::Or => {
                let mut result = Vec::new();
                for f in &self.filters {
                    for unit in f.apply(units) {
                        if !result.contains(&unit) {
                            result.push(unit);
                        }
                    }
                }
                result
            }
            FilterLogic::Not => {
                let filtered = self.filters[0].apply(units);
                units.iter()
                    .filter(|u| !filtered.contains(u))
                    .cloned()
                    .collect()
            }
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


