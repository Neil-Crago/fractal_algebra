//! Defines a comprehensive filtering system for `Fractal` objects and `SemanticUnit`s.
//!
//! This module provides the `ResonanceFilter` trait and a variety of implementations
//! for selecting fractals based on different criteria like resonance laws, scores, tags,
//! and metadata. It also includes logic for composing filters together.

use crate::resonance::{ResonanceFilter, ResonanceLaw, SemanticUnit};
use crate::traits::{Fractal, FractalCollection};
use std::any::Any;

// --- Simple Filters ---

/// Filters fractals based on a list of allowed `ResonanceLaw`s.
pub struct LawFilter {
    pub allowed: Vec<ResonanceLaw>,
}

impl ResonanceFilter for LawFilter {
    fn as_any(&self) -> &dyn Any { self }

    /// `passes` checks the `ResonanceLaw` of the `Fractal` trait object directly.
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        self.allowed.contains(&fractal.resonance_law())
    }

    /// `apply` filters `SemanticUnit`s. Since `ResonanceLaw` is not directly on `SemanticUnit`,
    /// this implementation uses the unit's `label` as a proxy, assuming a naming convention.
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units
            .iter()
            .filter(|u| self.allowed.iter().any(|law| u.label == law.to_string()))
            .cloned()
            .collect()
    }
}

/// Filters fractals based on a minimum resonance score.
pub struct ScoreFilter {
    pub min_score: f64,
}

impl ResonanceFilter for ScoreFilter {
    fn as_any(&self) -> &dyn Any { self }

    /// `passes` checks the resonance score of the `Fractal` trait object.
    fn passes(&self, fractal: &dyn Fractal) -> bool {
        fractal.resonance_score() >= self.min_score
    }

    /// `apply` filters `SemanticUnit`s. Lacking a 'score' field, this implementation
    /// uses the `phase` field as a stand-in for the score.
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units.iter().filter(|u| u.phase >= self.min_score).cloned().collect()
    }
}

/// A flexible filter that uses a provided closure to test `SemanticUnit`s.
pub struct PredicateFilter {
    // Added Send + Sync bounds to ensure the filter can be shared safely across threads.
    pub predicate: Box<dyn Fn(&SemanticUnit) -> bool + Send + Sync>,
}

impl ResonanceFilter for PredicateFilter {
    fn as_any(&self) -> &dyn Any { self }

    /// `passes` cannot use the predicate, as it operates on `&dyn Fractal`, not `SemanticUnit`.
    /// This implementation defaults to `true`, making it a no-op at the `Fractal` level.
    /// Its filtering logic is only effective when used via the `apply` method.
    fn passes(&self, _fractal: &dyn Fractal) -> bool {
        true
    }

    /// `apply` uses the stored predicate to filter a slice of `SemanticUnit`s.
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units.iter().filter(|&u| (self.predicate)(u)).cloned().collect()
    }
}

/// Filters based on whether a fractal has all of a given set of tags.
pub struct TagMatchFilter {
    pub required_tags: Vec<String>,
}

impl ResonanceFilter for TagMatchFilter {
    fn as_any(&self) -> &dyn Any { self }

    fn passes(&self, fractal: &dyn Fractal) -> bool {
        // Passes if every tag in `required_tags` is present in the fractal's tags.
        self.required_tags.iter().all(|tag| fractal.tags().contains(tag))
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        // Re-uses the `passes` logic on the semantic unit's underlying fractal.
        units.iter().filter(|unit| self.passes(&*unit.fractal)).cloned().collect()
    }
}

/// Filters based on the `domain` field in a fractal's metadata.
pub struct DomainFilter {
    pub domain: String,
}

impl ResonanceFilter for DomainFilter {
    fn as_any(&self) -> &dyn Any { self }

    fn passes(&self, fractal: &dyn Fractal) -> bool {
        fractal.metadata().domain == self.domain
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        units.iter().filter(|unit| self.passes(&*unit.fractal)).cloned().collect()
    }
}

// --- Composite Filters (Two Implementations) ---

// Note: This file contains two different implementations for composing filters:
// 1. `ComposedFilter` (struct) + `FilterLogic` (enum)
// 2. `CompositeFilter` (enum)
// The enum-based approach (`CompositeFilter`) is generally more idiomatic in Rust.
// Consider consolidating to a single approach.

/// The logic for combining filters in `ComposedFilter`.
#[derive(Debug, Clone)]
pub enum FilterLogic {
    And,
    Or,
    Not,
}

/// A filter that combines multiple sub-filters using a specified logic.
/// Note: This is one of two composite filter implementations in this file.
pub struct ComposedFilter {
    pub filters: Vec<Box<dyn ResonanceFilter>>,
    pub logic: FilterLogic,
}

impl ResonanceFilter for ComposedFilter {
    fn as_any(&self) -> &dyn Any { self }

    fn passes(&self, fractal: &dyn Fractal) -> bool {
        match self.logic {
            // Passes if ALL sub-filters pass.
            FilterLogic::And => self.filters.iter().all(|f| f.passes(fractal)),
            // Passes if ANY sub-filter passes.
            FilterLogic::Or => self.filters.iter().any(|f| f.passes(fractal)),
            // Passes if the first filter does NOT pass (assumes [0] exists).
            FilterLogic::Not => !self.filters[0].passes(fractal),
        }
    }

    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        match self.logic {
            // Applies filters sequentially, passing the result of one to the next (a pipeline).
            FilterLogic::And => self.filters.iter().fold(units.to_vec(), |acc, f| f.apply(&acc)),
            // Applies each filter to the original set and collects all unique results.
            FilterLogic::Or => {
                let mut result = Vec::new();
                for f in &self.filters {
                    for unit in f.apply(units) {
                        if !result.contains(&unit) { // Naive uniqueness check
                            result.push(unit);
                        }
                    }
                }
                result
            }
            // Returns all units that are NOT present in the result of the first filter.
            FilterLogic::Not => {
                let filtered_out = self.filters[0].apply(units);
                units
                    .iter()
                    .filter(|u| !filtered_out.contains(u))
                    .cloned()
                    .collect()
            }
        }
    }
}

/// An enum-based approach to composing filters. This is often more flexible and idiomatic.
/// Note: This is the second of two composite filter implementations in this file.
pub enum CompositeFilter {
    /// Passes if all sub-filters pass.
    All(Vec<Box<dyn ResonanceFilter>>),
    /// Passes if any sub-filter passes.
    Any(Vec<Box<dyn ResonanceFilter>>),
    /// Passes if the sub-filter does not pass.
    Not(Box<dyn ResonanceFilter>),
}

impl ResonanceFilter for CompositeFilter {
    fn as_any(&self) -> &dyn Any { self }

    fn passes(&self, fractal: &dyn Fractal) -> bool {
        match self {
            CompositeFilter::All(filters) => filters.iter().all(|f| f.passes(fractal)),
            CompositeFilter::Any(filters) => filters.iter().any(|f| f.passes(fractal)),
            CompositeFilter::Not(filter) => !filter.passes(fractal),
        }
    }
    
    fn apply(&self, units: &[SemanticUnit]) -> Vec<SemanticUnit> {
        // This implementation of apply simply re-uses the `passes` logic.
        // This is simpler than `ComposedFilter::apply` but may be less flexible,
        // as it doesn't pipeline `And` or merge `Or` results.
        units
            .iter()
            .filter(|unit| self.passes(&*unit.fractal))
            .cloned()
            .collect()
    }
}

// --- Filtering Infrastructure ---

/// A record of which fractals passed or failed a named filter. Useful for debugging.
#[derive(Debug, Clone)]
pub struct FilterTrace {
    pub filter_name: String,
    pub passed: Vec<usize>, // Indices of fractals that passed
    pub failed: Vec<usize>, // Indices that failed
}

/// A container that augments a `FractalCollection` with pre-calculated resonance data.
/// This avoids re-computing scores and laws repeatedly during filtering operations.
#[derive(Debug, Clone)]
pub struct ResonantFractalCollection {
    pub collection: FractalCollection,
    pub resonance_scores: Vec<f64>,
    pub resonance_laws: Vec<ResonanceLaw>,
    pub average_resonance: f64,
}

impl ResonantFractalCollection {
    /// Creates a new `ResonantFractalCollection` by calculating resonance data
    /// from the members of the base `FractalCollection`.
    pub fn new(collection: FractalCollection) -> Self {
        let resonance_scores: Vec<f64> = collection
            .members
            .iter()
            .map(|m| m.fractal.resonance_score())
            .collect();

        let resonance_laws: Vec<ResonanceLaw> = collection
            .members
            .iter()
            .map(|m| m.fractal.resonance_law())
            .collect();

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

    /// Re-evaluates all resonance data in place.
    /// This is useful after the collection has been mutated.
    pub fn reevaluate(&mut self) {
        *self = Self::new(self.collection.clone());
    }
}

/// Filtering methods for `ResonantFractalCollection`.
impl ResonantFractalCollection {
    /// Applies a filter's `passes` method to the collection, returning a new, smaller
    /// `FractalCollection` containing only the members that passed.
    pub fn filter(&self, filter: &dyn ResonanceFilter) -> FractalCollection {
        let filtered_members = self
            .collection
            .members
            .iter()
            .filter(|m| filter.passes(&m.fractal)) // Uses the `passes` method
            .cloned()
            .collect();

        FractalCollection { members: filtered_members }
    }

    /// Runs a filter's `passes` method over the collection and returns a `FilterTrace`
    /// detailing which member indices passed or failed.
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