//! Defines the core traits and data structures for the quantum-inspired AI system.
//!
//! This module lays the groundwork for the simulation, establishing the concepts of
//! an `EntangledSystem` (the environment), `ParticleResonance` (the actors),
//! and the actions (`EntropyPulse`) and feedback (`FeedbackSignal`) that drive the learning process.

use crate::graph::{FractalGraph, NodeId};
use num_complex::Complex;

/// Represents the entire computational environment, linking a fractal graph structure
/// to a pair of entangled, particle-like resonance patterns.
///
/// This struct serves as the primary container for the simulation, where the `FractalGraph`
/// acts as the substrate for quantum-inspired computations.
pub struct EntangledSystem {
    /// The underlying graph structure where computations occur. The nodes and edges
    /// hold complex numbers to represent quantum-like amplitudes and phases.
    pub graph: FractalGraph<Complex<f32>>,
    /// A pair of entangled particles, represented by resonance patterns on the graph.
    pub particles: (ParticleResonance, ParticleResonance),
}

impl EntangledSystem {
    /// Creates a new `EntangledSystem` with a given graph and two initial particles.
    pub fn new(
        graph: FractalGraph<Complex<f32>>,
        particle_a: ParticleResonance,
        particle_b: ParticleResonance,
    ) -> Self {
        Self {
            graph,
            particles: (particle_a, particle_b),
        }
    }

    /// (Internal utility) Creates a particle representation at a specific node in the graph.
    ///
    /// This function initializes a resonance pattern by setting the weights of all edges
    /// connected to the `node_id` to a base state (e.g., a complex value of 1.0 + 0.0i).
    /// It is marked as `dead_code` for now, as its usage may be part of future development.
    #[allow(dead_code)]
    fn create_particle_at(&mut self, node_id: NodeId) -> Option<ParticleResonance> {
        // Safely get mutable access to the edges connected to the target node.
        if let Some(connected_edges) = self.graph.get_edges_for_node_mut(node_id) {
            // Set the state for each connected edge to represent the particle's base state.
            for edge in connected_edges.iter_mut() {
                // The weight represents the quantum amplitude/phase. Here, we initialize it.
                edge.weight = Complex::new(1.0, 0.0); // Real amplitude 1.0, phase 0.
            }

            // Collect the destination nodes of these edges, which define the particle's pattern.
            let state_edge_destinations: Vec<NodeId> =
                connected_edges.iter().map(|edge| edge.destination).collect();

            // Return the newly defined particle.
            Some(ParticleResonance {
                pattern_nodes: state_edge_destinations.clone(),
                core_node: node_id,
                state_edges: state_edge_destinations,
            })
        } else {
            // If the node doesn't exist, we cannot create a particle there.
            None
        }
    }
}

/// Represents a particle as a stable, oscillating pattern of states within the `FractalGraph`.
///
/// A particle is not a single point but a subgraph of resonating nodes and edges,
/// identified by a central `core_node`.
pub struct ParticleResonance {
    /// The set of nodes that form the particle's resonance pattern.
    pub pattern_nodes: Vec<NodeId>,
    /// The central node from which the particle pattern originates.
    pub core_node: NodeId,
    /// The specific edges whose weights (amplitudes) oscillate to represent the particle's state.
    pub state_edges: Vec<NodeId>,
}

/// Represents an action performed on the `EntangledSystem` to probe its state.
///
/// This is the "guess" that the AI makes. It's a wave-like pulse intended to
/// interact with the `ParticleResonance` patterns.
#[derive(Clone)]
pub struct EntropyPulse {
    pub frequency: f64,
    pub amplitude: f64,
    pub waveform: String, // e.g., "sine", "square"
}

/// Represents the result of a measurement, used as feedback for the learning algorithm.
///
/// After an `EntropyPulse` is applied, the system yields a `FeedbackSignal` that
/// tells the AI how "good" its guess was.
#[derive(Clone, Debug)]
pub struct FeedbackSignal {
    /// A metric indicating the quality of the last `EntropyPulse`.
    /// The AI's objective is to minimize this value, driving it towards zero.
    /// A lower value means the pulse was closer to an ideal resonance.
    pub correlation_strength: f64,
}

/// Defines the behavior of a probabilistic, Bayesian-style learning engine.
///
/// This trait abstracts the mechanism that proposes new guesses (`EntropyPulse`) and
/// learns from the results (`FeedbackSignal`).
pub trait ProbabilisticSearch {
    /// Proposes a new `EntropyPulse` based on the agent's current beliefs.
    /// This is the "exploration" step, where the AI makes an educated guess.
    fn propose_best_guess(&self) -> EntropyPulse;

    /// Updates the agent's internal beliefs based on the feedback from the last guess.
    /// This is the "learning" step.
    fn update(&mut self, feedback: &FeedbackSignal, last_guess: &EntropyPulse);
}

/// Defines the behavior of a symmetry constraint engine.
///
/// This trait can be used to enforce specific rules or symmetries on the system,
/// ensuring that proposed actions are physically or logically valid.
pub trait SymmetryConstraint {
    /// Checks if a proposed `EntropyPulse` adheres to the system's constraints.
    fn is_valid(&self, pulse: &EntropyPulse) -> bool;
}