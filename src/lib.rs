pub mod edge;
pub mod field;
pub mod traits;
pub mod vec3;
pub mod algebra;
pub mod laws;

pub use edge::FractalEdge;
pub use field::FractalField;
pub use traits::FractalAlgebra;
pub use vec3::Vec3;
pub use laws::{test_associativity, test_distributivity};
