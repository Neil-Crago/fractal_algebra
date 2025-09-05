//! # Fractal Algebra & Generative Systems
//!
//! `fractal_algebra` is a Rust library for exploring generative art and complex systems
//! through the lens of algebraic structures and quantum-inspired concepts.
//!
//! The core idea is to represent complex patterns as `FractalField` objects, which
//! behave like elements in a vector space. These fields can be generated, mutated,
//! and evaluated through an evolutionary process managed by the `GeneratorCriticLoop`.
//!
//! This library integrates My original research concept of a `FractalGraph` enriched
//! with quantum-inspired amplitudes, making it the foundational data structure for
//! advanced simulations within an `EntangledSystem`.
//!
//! ## Key Components
//!
//! - **`FractalField`**: The central data structure, representing a collection of geometric
//!   `GraphEdge`s. It supports algebraic operations like addition and scalar multiplication.
//! - **`Generator` & `Critic` Traits**: A flexible system for creating and evaluating
//!   `FractalField`s. Use `EvolutionaryGenerator` and `CriticSuite` for a standard setup.
//! - **`GeneratorCriticLoop`**: An engine that drives the evolutionary process, iteratively
//!   finding better fields based on critic scores.
//! - **`Resonance` Trait**: An abstraction for measuring coherence, harmony, and other
//!   emergent properties of fractals and their transformations.
//! - **`FractalGraph`**: A semantically-typed, directed graph that serves as the substrate
//!   for quantum-inspired computations and particle simulations.
//! - **AI & Bayesian Search**: Modules like `ai` and `bayes` provide tools for probabilistic
//!   searches and simulations within an `EntangledSystem`.
//!
//! ## Example Usage
//!
//! ```no_run
//! use fractal_algebra::{
//!     GeneratorCriticLoop, EvolutionaryGenerator, MutationSuite, CriticSuite,
//!     SymmetryCritic, EntropyCritic, FractalField
//! };
//!
//! // 1. Set up a generator with mutation strategies.
//! let generator = EvolutionaryGenerator {
//!     mutations: MutationSuite::new(), // Add strategies here
//!     count: 10,
//! };
//!
//! // 2. Set up critics to evaluate the fields.
//! let mut critics = CriticSuite::new();
//! critics.add_critic(SymmetryCritic, 0.7);
//! critics.add_critic(EntropyCritic, 0.3);
//!
//! // 3. Create and run the evolutionary loop.
//! let loop_engine = GeneratorCriticLoop {
//!     generator,
//!     critic_suite: critics,
//!     iterations: 100,
//! };
//!
//! if let Some(best_field) = loop_engine.run() {
//!     println!("Found a best field!");
//!     // Further analysis on best_field...
//! }
//! ```

// --- Module Declarations ---
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
pub mod ai;
pub mod bayes;

#[macro_use]
mod macros;


// --- Public API Exports ---

// Core algebraic and geometric types
pub use constants::MODULUS;
pub use field::FractalField;
pub use fractaledge::FractalEdge;
pub use graphedge::GraphEdge;
pub use signature::FractalSignature;
pub use vec3::Vec3;

// Graph-related types
pub use graph::{FractalGraph, FractalGraphEdge, EdgeType, GraphError, NodeId};

// Evolutionary loop components
pub use criticloop::GeneratorCriticLoop;
pub use critics::CriticSuite;
pub use evolutionary::EvolutionaryGenerator;
pub use looprep::LoopReport;
pub use mutation::MutationSuite;
pub use rfg::RandomFieldGenerator;

// AI and Quantum-Inspired components
pub use ai::{EntangledSystem, EntropyPulse, FeedbackSignal, ParticleResonance, ProbabilisticSearch};
pub use bayes::{FrequencyBeliefSpace, Gaussian};
pub use atom::{FractalAtom};

// Resonance and Transformation framework
pub use resonance::{
    Resonance, ResonanceFilter, ResonanceLaw, ResonantTransform,
    TransformResonanceLaw,
};
pub use filters::{FilterTrace, LawFilter, PredicateFilter, ScoreFilter};

// Core Traits
pub use traits::{
    CollectionMember, Critic, EntropyCritic, Fractal, FractalClone,
    FractalCollection, Generator, HasSignature, IFS, Mandelbrot, MutationStrategy,
    Operation, SymmetryCritic,
};

// Spacetime simulation types
pub use time::{Evolvable, FractalSpacetime, SpacetimeCoordinate};

// Testing utilities and laws
pub use laws::{test_associativity, test_distributivity};
pub use testkit::canonical_test_fractal;