#[macro_export]
macro_rules! test_vector_space_axioms {
    ($field:expr, $scalar:expr) => {{
        let zero = FractalField::zero();
        let f = $field;
        let s = $scalar;

        assert_eq!(f.clone() + zero.clone(), f);
        assert_eq!(zero.clone() + f.clone(), f);
        assert_eq!(f.clone() + (-f.clone()), zero);
        assert_eq!(f.clone() * Complex::new(1.0, 0.0), f);
        assert_eq!(f.clone() * Complex::new(0.0, 0.0), zero);
        assert_eq!(f.clone() * s + f.clone() * s, f.clone() * (s + s));
    }};
}
