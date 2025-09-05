//! Contains unit tests and algebraic law verification functions.

use crate::field::FractalField;
use crate::testkit::canonical_test_fractal;
use num_complex::Complex;

// --- Algebraic Law Verifiers ---
// These functions verify that FractalField behaves like a proper vector space.

/// Verifies the associativity of addition: `(a + b) + c == a + (b + c)`.
pub fn test_add_associativity() -> bool {
    let a = canonical_test_fractal();
    let b = a.clone() * Complex::new(2.0, 0.0);
    let c = a.clone() * Complex::new(3.0, 0.0);
    // clone() is necessary because the `+` operator consumes its operands.
    (a.clone() + b.clone()) + c.clone() == a + (b + c)
}

/// Verifies the commutativity of addition: `a + b == b + a`.
pub fn test_add_commutativity() -> bool {
    let a = canonical_test_fractal();
    let b = a.clone() * Complex::new(2.0, 0.0);
    a.clone() + b.clone() == b + a
}

/// Verifies the additive identity: `a + 0 == a`.
pub fn test_add_identity() -> bool {
    let a = canonical_test_fractal();
    let zero = FractalField::zero();
    a.clone() + zero == a
}

/// Verifies the additive inverse: `a + (-a) == 0`.
pub fn test_add_inverse() -> bool {
    let a = canonical_test_fractal();
    let sum = a.clone() + -a; // The `-` operator calls the Neg trait.
    sum.is_zero()
}

// --- Unit Tests for other modules ---
#[cfg(test)]
mod tests {
    use crate::atom::{TagSet, TagSetError};

    #[test]
    fn new_empty_collection_returns_error() {
        let input: Vec<String> = vec![];
        assert_eq!(TagSet::new(input), Err(TagSetError::EmptyCollection));
    }

    #[test]
    fn new_with_empty_string_tag_returns_error() {
        let input = vec!["".to_string()];
        assert_eq!(TagSet::new(input), Err(TagSetError::EmptyTag("".to_string())));
    }

    #[test]
    fn new_with_whitespace_only_tag_returns_error() {
        let input = vec!["   ".to_string()];
        assert_eq!(TagSet::new(input), Err(TagSetError::EmptyTag("".to_string())));
    }

    #[test]
    fn new_with_duplicate_tags_returns_error() {
        let input = vec!["apple".to_string(), "banana".to_string(), "apple".to_string()];
        assert_eq!(TagSet::new(input), Err(TagSetError::DuplicateTag("apple".to_string())));
    }

    #[test]
    fn new_sorts_tags_successfully() {
        let raw = vec!["  zeta ".to_string(), "alpha".to_string(), "beta".to_string()];
        let tags = TagSet::new(raw).expect("should construct successfully");
        let collected: Vec<String> = tags.iter().cloned().collect();
        assert_eq!(collected, vec!["alpha", "beta", "zeta"]);
    }
}