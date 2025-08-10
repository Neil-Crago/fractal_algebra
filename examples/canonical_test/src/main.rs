use fractal_algebra::FractalAlgebra;
use fractal_algebra::field::FractalField;
use fractal_algebra::test_vector_space_axioms;
use fractal_algebra::testkit::canonical_test_fractal;
use fractal_algebra::tests::{
    test_add_associativity, test_add_commutativity, test_add_identity, test_for_distributivity,
};
use num_complex::Complex;

fn main() {
    let f = canonical_test_fractal();
    let g = f.scale(Complex::new(2.0, 0.0));
    let h = f.add(&g);

    println!("\ncanonical_test_fractal() => {f:?}\n\n");
    println!("f.scale(2.0, 0.0) => {g:?}\n\n");
    println!("f.add(g) => {h:?}");

    assert!(test_add_associativity());
    assert!(test_add_commutativity());
    assert!(test_add_identity());
    
    // assert!(test_add_inverse());
    let f = FractalField::one();
    let sum = f.clone() + (-f.clone());
    assert!(sum.is_zero());

    // assert!(test_mul_identity());
    let f = FractalField::one();
    let product = f.clone() * Complex::new(1.0, 0.0);
    assert_eq!(product, f);

    assert!(test_for_distributivity());

    println!("All ring axioms passed.");

    test_vector_space_axioms!(FractalField::one(), Complex::new(2.0, -1.0));
}
