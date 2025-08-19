pub mod atom;
pub mod constants;
pub mod criticloop;
pub mod critics;
pub mod evolutionary;
pub mod field;
pub mod filters;
pub mod fractaledge;
pub mod graph;
pub mod graphedge;
pub mod laws;
pub mod looprep;
pub mod mutation;
pub mod resonance;
pub mod rfg;
pub mod signature;
pub mod stochastic;
pub mod testkit;
pub mod tests;
pub mod time;
pub mod traits;
pub mod vec3;

pub use constants::MODULUS;
pub use criticloop::GeneratorCriticLoop;
pub use critics::CriticSuite;
pub use evolutionary::EvolutionaryGenerator;
pub use field::FractalField;
pub use fractaledge::FractalEdge;
pub use graphedge::GraphEdge;
pub use laws::{test_associativity, test_distributivity};
pub use looprep::LoopReport;
pub use mutation::MutationSuite;
pub use rfg::RandomFieldGenerator;
pub use signature::FractalSignature;
pub use stochastic::StochasticAmplitudePhase;
pub use testkit::canonical_test_fractal;

pub use tests::{
    test_add_associativity, test_add_commutativity, test_add_identity, test_add_inverse,
    test_for_distributivity, test_mul_identity,
};
pub use traits::{
    CollectionMember, Critic, EntropyCritic, Fractal, FractalAlgebra, FractalClone,
    FractalCollection, FractalRing, Generator, HasSignature, IFS, Mandelbrot, MutationStrategy,
    Operation, SymmetryCritic, add_fractals, mul_fractals, sub_fractals,
};

pub use graph::{FactorialNode, FractalGraph};
pub use vec3::Vec3;

pub use time::{Evolvable, FractalSpacetime, SpacetimeCoordinate};

pub use resonance::{
    CompositeTransform, Resonance, ResonanceFilter, ResonanceLaw, ResonantTransform,
    TransformResonanceLaw,
};

pub use filters::{FilterTrace, LawFilter, PredicateFilter, ScoreFilter};

pub use atom::FractalAtom;

#[macro_use]
mod macros;
