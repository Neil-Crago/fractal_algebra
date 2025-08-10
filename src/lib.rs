pub mod algebra;
pub mod constants;
pub mod field;
pub mod fractaledge;
pub mod graphedge;
pub mod laws;
pub mod testkit;
pub mod tests;
pub mod traits;
pub mod vec3;

pub use constants::MODULUS;
pub use field::FractalField;
pub use fractaledge::FractalEdge;
pub use graphedge::GraphEdge;
pub use laws::{test_associativity, test_distributivity};
pub use testkit::canonical_test_fractal;
pub use tests::{
    test_add_associativity, test_add_commutativity, test_add_identity, test_add_inverse,
    test_for_distributivity, test_mul_identity,
};
pub use traits::{FractalAlgebra, FractalRing};
pub use vec3::Vec3;

#[macro_use]
mod macros;
