use fractal_algebra::testkit::canonical_test_fractal;
use num_complex::Complex;
use fractal_algebra::FractalAlgebra;
use fractal_algebra::tests::{test_add_associativity,
                            test_add_commutativity,
                            test_add_identity,
                            test_add_inverse,
                            test_mul_identity,
                            test_for_distributivity};


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
    assert!(test_add_inverse());
    assert!(test_mul_identity());
    assert!(test_for_distributivity());

    println!("All ring axioms passed.");
}
