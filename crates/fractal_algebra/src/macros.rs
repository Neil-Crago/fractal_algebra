//! Provides macros for reducing boilerplate in tests.

/// A macro to test that a `FractalField` satisfies basic vector space axioms.
///
/// This macro generates several `assert_eq!` statements to check for:
/// - Additive identity (f + 0 = f)
/// - Additive inverse (f + (-f) = 0)
/// - Multiplicative identity (f * 1 = f)
/// - Multiplication by zero (f * 0 = 0)
///
/// # Usage
///
/// ```
/// # use fractal_algebra::{FractalField, test_vector_space_axioms};
/// # use num_complex::Complex;
/// #[test]
/// fn test_my_field() {
///     let my_field = FractalField::one();
///     let my_scalar = Complex::new(2.0, 3.0);
///     test_vector_space_axioms!(my_field, my_scalar);
/// }
/// ```
#[macro_export]
macro_rules! test_vector_space_axioms {
    ($field:expr, $scalar:expr) => {{
        use num_complex::Complex;
        use $crate::FractalField; // Use crate-relative path

        let zero = FractalField::zero();
        let f = $field;
        let s = $scalar;

        // Test additive identity
        assert_eq!(f.clone() + zero.clone(), f);
        assert_eq!(zero.clone() + f.clone(), f);
        // Test additive inverse
        assert_eq!(f.clone() + (-f.clone()), zero);
        // Test multiplicative identity
        assert_eq!(f.clone() * Complex::new(1.0, 0.0), f);
        // Test multiplication by zero scalar
        assert_eq!(f.clone() * Complex::new(0.0, 0.0), zero);
        // A simple distributivity check (Note: this is just one case)
        assert_eq!(f.clone() * s + f.clone() * s, f.clone() * (s + s));
    }};
}