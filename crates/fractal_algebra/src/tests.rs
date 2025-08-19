#[cfg(test)]
use crate::atom::{TagSet, TagSetError};
use crate::field::FractalField;
use crate::testkit::canonical_test_fractal;
use crate::traits::FractalAlgebra;
use num_complex::Complex;

pub fn test_add_associativity() -> bool {
    let a = canonical_test_fractal();
    let b = a.scale(Complex::new(2.0, 0.0));
    let c = a.scale(Complex::new(3.0, 0.0));

    let left = a.add(&b.add(&c));
    let right = a.add(&b).add(&c);
    left.edges == right.edges
}

pub fn test_add_commutativity() -> bool {
    let a = canonical_test_fractal();
    let b = a.scale(Complex::new(2.0, 0.0));

    a.add(&b).edges == b.add(&a).edges
}

pub fn test_add_identity() -> bool {
    let a = canonical_test_fractal();
    let zero = FractalField::zero();

    a.add(&zero).edges == a.edges
}

pub fn test_add_inverse() -> bool {
    let a = canonical_test_fractal();
    let neg = a.negate();
    let sum = a.add(&neg);

    sum.edges.is_empty()
}

pub fn test_mul_identity() -> bool {
    let a = canonical_test_fractal();
    let one = FractalField::one();

    a.mul(&one).edges == a.edges
}

pub fn test_for_distributivity() -> bool {
    let a = canonical_test_fractal();
    let b = a.scale(Complex::new(2.0, 0.0));
    let c = a.scale(Complex::new(3.0, 0.0));

    let left = a.mul(&b.add(&c));
    let right = a.mul(&b).add(&a.mul(&c));

    left.edges == right.edges
}

#[cfg(test)]
mod tests {
    use super::{TagSet, TagSetError};

    #[test]
    fn new_empty_collection_returns_error() {
        let input: Vec<String> = vec![];
        let result = TagSet::new(input);
        assert_eq!(result, Err(TagSetError::EmptyCollection));
    }

    #[test]
    fn new_with_empty_string_tag_returns_error() {
        let input = vec!["".to_string()];
        let result = TagSet::new(input);
        // trimmed tag is "", so we expect an EmptyTag("") error
        assert_eq!(result, Err(TagSetError::EmptyTag("".to_string())));
    }

    #[test]
    fn new_with_whitespace_only_tag_returns_error() {
        let input = vec!["   ".to_string()];
        let result = TagSet::new(input);
        // trimmed tag is "", treated the same as an empty tag
        assert_eq!(result, Err(TagSetError::EmptyTag("".to_string())));
    }

    #[test]
    fn new_with_duplicate_tags_returns_error() {
        let input = vec![
            "apple".to_string(),
            "banana".to_string(),
            "apple".to_string(),
        ];
        let result = TagSet::new(input);
        // After sort, duplicates are detected on "apple"
        assert_eq!(result, Err(TagSetError::DuplicateTag("apple".to_string())));
    }

    #[test]
    fn new_sorts_and_dedupes_tags_successfully() {
        let raw = vec![
            "  zeta ".to_string(),
            "alpha".to_string(),
            "beta".to_string(),
        ];
        let tags = TagSet::new(raw).expect("should construct successfully");
        let collected: Vec<String> = tags.iter().cloned().collect();
        assert_eq!(collected, vec!["alpha", "beta", "zeta"]);
    }

    #[test]
    fn contains_detects_existing_and_missing_tags() {
        let tags = TagSet::new(vec!["one".to_string(), "two".to_string()]).unwrap();
        assert!(tags.contains("one"));
        assert!(tags.contains("two"));
        assert!(!tags.contains("three"));
    }

    #[test]
    fn len_and_is_empty_behave_correctly() {
        let single = TagSet::new(vec!["solo".to_string()]).unwrap();
        assert_eq!(single.len(), 1);
        assert!(!single.is_empty());

        // Even after into_iter, original semantics hold
        let mut iter = single.clone().into_iter();
        assert_eq!(iter.next(), Some("solo".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter_returns_all_tags_in_order() {
        let tags = TagSet::new(vec!["b".to_string(), "a".to_string()]).unwrap();
        let collected: Vec<String> = tags.into_iter().collect();
        assert_eq!(collected, vec!["a", "b"]);
    }
}
