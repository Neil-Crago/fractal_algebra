use fractal_algebra::{FractalGraph, NodeId};
use num_complex::Complex;

/// The main container for our quantum experiment.
pub struct EntangledSystem {
    pub graph: FractalGraph<Complex<f32>>,
    pub particles: (ParticleResonance, ParticleResonance),
}

impl EntangledSystem {
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
    #[allow(dead_code)]
    fn create_particle_at(&mut self, node_id: NodeId) -> Option<ParticleResonance> {
        // 1. Get the mutable list of edges for the central node.
        //    We use `if let Some(...)` to safely handle the case where the node might not exist.
        if let Some(connected_edges) = self.graph.get_edges_for_node_mut(node_id) {
            // 2. Iterate through the edges mutably and set their state.
            for edge in connected_edges.iter_mut() {
                // Directly assign to the public `weight` field.
                edge.weight = Complex::new(1.0, 0.0); // e.g., phase = 0
            }

            // 3. Collect the destination NodeIds of the edges that define the particle's state.
            let state_edge_destinations: Vec<NodeId> = connected_edges
                .iter()
                .map(|edge| edge.destination)
                .collect();

            // 4. Return the newly created particle representation.
            Some(ParticleResonance {
                pattern_nodes: state_edge_destinations.clone(),
                core_node: node_id,
                state_edges: state_edge_destinations,
            })
        } else {
            // If the node doesn't exist or has no edges, we can't create a particle.
            None
        }
    }
}

/// A particle, represented as a stable, oscillating pattern.
pub struct ParticleResonance {
    // Defines the subgraph/pattern that constitutes the particle
    pub pattern_nodes: Vec<NodeId>,
    pub core_node: NodeId,
    pub state_edges: Vec<NodeId>, // Edges whose weights oscillate to represent the particle's state
}

/// The "action" we perform on the system.
#[derive(Clone)]
pub struct EntropyPulse {
    pub frequency: f64,
    pub amplitude: f64,
    pub waveform: String, // e.g., "sine", "square"
}

/// The result of a measurement, used as feedback.
#[derive(Clone)]
pub struct FeedbackSignal {
    pub correlation_strength: f64, // e.g., -1.0 to 1.0
    pub phase_alignment_error: f64,
}
