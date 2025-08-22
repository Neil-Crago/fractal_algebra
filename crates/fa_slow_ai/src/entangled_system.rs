//! This module defines the core simulation environment, the `EntangledSystem`.
//! It contains the graph representing spacetime and the particles within it.

use fa_core::state::{EntropyPulse, FeedbackSignal}; // Import EntropyPulse and FeedbackSignal
use fractal_algebra::graph::{EdgeType, FractalGraph, FractalGraphEdge, NodeId};
use num_complex::Complex;
use rand::Rng;
use std::collections::HashMap;

// --- Data Structures ---

/// Represents a particle as a stable, resonant pattern in the graph.
#[derive(Debug, Clone)]
pub struct ParticleResonance {
    /// The central node of the particle pattern.
    pub core_node: NodeId,
    /// The destination nodes of the edges that define the particle's state.
    /// The state itself is stored as the complex weight on these edges.
    pub state_edges: Vec<NodeId>,
}

/// The main container for our quantum experiment.
pub struct EntangledSystem {
    pub graph: FractalGraph<()>, // Node payload is empty for now
    pub particles: Vec<ParticleResonance>,
    // We can store grid dimensions for easy reference
    grid_width: u64,
    grid_height: u64,
}

// --- Implementation ---

impl EntangledSystem {
    /// Creates a new simulation environment with two entangled particles.
    pub fn new(width: u64, height: u64) -> Self {
        let mut graph = FractalGraph::new();
        let mut node_map = HashMap::new(); // Helper to find NodeIds from (x, y) coords

        // 1. Create a 2D grid of nodes to represent our "space".
        for y in 0..height {
            for x in 0..width {
                let node_id = graph.add_node(()); // Empty payload
                node_map.insert((x, y), node_id);
            }
        }

        // 2. Connect adjacent nodes with default edges.
        for y in 0..height {
            for x in 0..width {
                let current_node = node_map[&(x, y)];
                // Connect to the node on the right
                if x + 1 < width {
                    let right_node = node_map[&(x + 1, y)];
                    graph
                        .add_edge(
                            current_node,
                            right_node,
                            EdgeType::Resonant,
                            Complex::new(0.0, 0.0),
                        )
                        .unwrap();
                    graph
                        .add_edge(
                            right_node,
                            current_node,
                            EdgeType::Resonant,
                            Complex::new(0.0, 0.0),
                        )
                        .unwrap();
                }
                // Connect to the node below
                if y + 1 < height {
                    let bottom_node = node_map[&(x, y + 1)];
                    graph
                        .add_edge(
                            current_node,
                            bottom_node,
                            EdgeType::Resonant,
                            Complex::new(0.0, 0.0),
                        )
                        .unwrap();
                    graph
                        .add_edge(
                            bottom_node,
                            current_node,
                            EdgeType::Resonant,
                            Complex::new(0.0, 0.0),
                        )
                        .unwrap();
                }
            }
        }

        let mut system = Self {
            graph,
            particles: Vec::new(),
            grid_width: width,
            grid_height: height,
        };

        // 3. Create two particles at different locations.
        let particle_a_pos = (width / 4, height / 2);
        let particle_b_pos = (width * 3 / 4, height / 2);

        if let Some(node_a_id) = node_map.get(&particle_a_pos) {
            if let Some(particle_a) = system.create_particle_at(*node_a_id) {
                system.particles.push(particle_a);
            }
        }
        if let Some(node_b_id) = node_map.get(&particle_b_pos) {
            if let Some(particle_b) = system.create_particle_at(*node_b_id) {
                system.particles.push(particle_b);
            }
        }

        // 4. Entangle the two particles.
        if system.particles.len() == 2 {
            system.entangle(0, 1);
        }

        system
    }

    /// Creates a "particle" at a given node by initializing its local edge states.
    fn create_particle_at(&mut self, node_id: NodeId) -> Option<ParticleResonance> {
        // Get the mutable list of edges for the central node.
        if let Some(connected_edges) = self.graph.get_edges_for_node_mut(node_id) {
            // Set these specific edges to an initial "oscillating" state.
            for edge in connected_edges.iter_mut() {
                // The particle's initial state is a complex number with magnitude 1, phase 0.
                edge.weight = Complex::new(1.0, 0.0);
            }

            // Collect the destination NodeIds of the edges that define the particle's state.
            let state_edge_destinations: Vec<NodeId> = connected_edges
                .iter()
                .map(|edge| edge.destination)
                .collect();

            // Return the newly created particle representation.
            Some(ParticleResonance {
                core_node: node_id,
                state_edges: state_edge_destinations,
            })
        } else {
            // If the node doesn't exist or has no edges, we can't create a particle.
            None
        }
    }

    /// Establishes entanglement between two particles by creating a phase-lock.
    fn entangle(&mut self, particle_a_idx: usize, particle_b_idx: usize) {
        // This function establishes a simple anti-correlation (a Bell state).
        // Particle A was initialized with phase 0 (Complex::new(1.0, 0.0)).
        // We will now set Particle B's state to have the opposite phase.

        let core_node_b = self.particles[particle_b_idx].core_node;

        if let Some(edges_b) = self.graph.get_edges_for_node_mut(core_node_b) {
            for edge in edges_b.iter_mut() {
                // Complex::new(-1.0, 0.0) represents a magnitude of 1 and a phase of pi radians.
                // This creates a perfect anti-correlation with particle A's state.
                edge.weight = Complex::new(-1.0, 0.0);
            }
        }
        //println!("Entangled particle {} and particle {} with opposite phases.", particle_a_idx, particle_b_idx);
    }

    /// Applies a pulse to the entire system, causing decoherence.
    // In fa_slow_ai/src/entangled_system.rs

    /// Applies a pulse to the entire system, causing decoherence.
    pub fn apply_pulse(&mut self, pulse: &EntropyPulse) {
        // Get a random number generator. You'll need to add `use rand::Rng;` at the top.
        let mut rng = rand::rng();

        // Iterate through every edge in the graph.
        for edge in self.graph.all_edges_mut() {
            // --- This is the new decoherence logic ---
            // 1. Add a small amount of random noise to the pulse's frequency for each edge.
            //    This represents the chaotic nature of the universe.
            let noise = rng.random_range(-0.1..0.1); // A small random perturbation
            let noisy_frequency = pulse.frequency + noise;

            // 2. Create the complex number representing the noisy pulse's effect.
            let pulse_effect = Complex::from_polar(pulse.amplitude as f32, noisy_frequency as f32);

            // 3. Apply the effect by multiplying the complex numbers.
            edge.weight *= pulse_effect;

            // 4. Normalize the magnitude back to 1.0 to keep the state valid.
            if edge.weight.norm_sqr() > 0.0 {
                edge.weight = edge.weight.unscale(edge.weight.norm());
            }
        }
    }

    /// Performs a simulated measurement and returns the correlation feedback.
    pub fn measure_correlation(&mut self) -> FeedbackSignal {
        if self.particles.len() < 2 {
            // Cannot measure if we don't have two particles.
            return FeedbackSignal {
                correlation_strength: -1.0, // Indicates an error or invalid state
                phase_alignment_error: std::f64::consts::PI,
            };
        }

        // --- 1. "Dimensional Violence": Collapse Particle A ---
        // Randomly choose whether to collapse to state "up" (phase 0) or "down" (phase pi).
        let collapse_to_up = rand::random::<bool>();
        let collapsed_state_a = if collapse_to_up {
            Complex::new(1.0, 0.0) // Phase 0
        } else {
            Complex::new(-1.0, 0.0) // Phase pi
        };

        let core_node_a = self.particles[0].core_node;
        if let Some(edges_a) = self.graph.get_edges_for_node_mut(core_node_a) {
            for edge in edges_a.iter_mut() {
                edge.weight = collapsed_state_a;
            }
        }

        // --- 2. Read the State of Particle B ---
        let core_node_b = self.particles[1].core_node;
        let mut average_state_b = Complex::new(0.0, 0.0);
        if let Some(edges_b) = self.graph.get_edges_for_node_mut(core_node_b) {
            for edge in edges_b.iter() {
                average_state_b += edge.weight;
            }
            // Normalize to get the average phase direction.
            if average_state_b.norm_sqr() > 0.0 {
                average_state_b = average_state_b.unscale(average_state_b.norm());
            }
        }

        // --- 3. Calculate Correlation ---
        // Because we entangled them with opposite phases, the expected state for B
        // is the opposite of A's collapsed state.
        let expected_state_b = -collapsed_state_a;

        // The correlation can be calculated as the cosine of the angle between the
        // measured state and the expected state. This is the real part of their dot product.
        let correlation = (average_state_b * expected_state_b.conj()).re;
        let phase_error = (average_state_b.arg() - expected_state_b.arg()).abs();

        FeedbackSignal {
            correlation_strength: correlation as f64,
            phase_alignment_error: phase_error as f64,
        }
    }
}
