use fractal_algebra::field::FractalField;

#[test]
fn test_commutativity_addition() {
    let a = FractalField::random();
    let b = FractalField::random();
    assert_eq!(a.clone() + b.clone(), b + a);
}
