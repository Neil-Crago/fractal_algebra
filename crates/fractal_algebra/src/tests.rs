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

    a.multiply(&one).edges == a.edges
}

pub fn test_for_distributivity() -> bool {
    let a = canonical_test_fractal();
    let b = a.scale(Complex::new(2.0, 0.0));
    let c = a.scale(Complex::new(3.0, 0.0));

    let left = a.multiply(&b.add(&c));
    let right = a.multiply(&b).add(&a.multiply(&c));

    left.edges == right.edges
}
