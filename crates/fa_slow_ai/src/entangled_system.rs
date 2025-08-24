//! This module defines the core simulation environment, the `EntangledSystem`.
//! It contains the graph representing spacetime and the particles within it.

use fa_core::state::{EntropyPulse, FeedbackSignal}; // Import EntropyPulse and FeedbackSignal
use fractal_algebra::graph::{EdgeType, FractalGraph, NodeId};
use num_complex::Complex;
use rand::Rng;
use std::collections::HashMap;
use std::collections::VecDeque;

// --- Data Structures ---

/// Represents a single, controllable wave source in the simulation.
pub struct PulseSource {
    /// The graph node index where the pulse originates.
    pub location_node_index: usize,

    /// The strength of the wave from this source (amplitude).
    pub amplitude: f64,

    /// The initial phase offset in radians.
    /// Controlling this allows us to create precise interference patterns.
    pub phase_offset: f64,
}

/// Represents a particle as a stable, resonant pattern in the graph.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParticleResonance {
    /// The central node of the particle pattern.
    pub core_node: NodeId,
    /// The destination nodes of the edges that define the particle's state.
    /// The state itself is stored as the complex weight on these edges.
    pub state_edges: Vec<NodeId>,
}
#[allow(dead_code)]
impl ParticleResonance {
    /// Creates a new ParticleResonance with default values.
    pub fn new() -> Self {
        Self {
            core_node: NodeId(0),
            state_edges: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EntangledSystem {
    pub graph: FractalGraph<()>, // Node payload is empty for now
    pub particles: Vec<ParticleResonance>,
    pub grid_width: u64,
    pub grid_height: u64,
    pub adjacency_list: Vec<Vec<usize>>,
    pub particle_states: Vec<f64>,
}

// --- Implementation ---
#[allow(dead_code)]
impl EntangledSystem {
    /// Creates a new simulation environment with two entangled particles.
    pub fn new(width: u64, height: u64, adjacency_list: Vec<Vec<usize>>) -> Self {
        let num_nodes = adjacency_list.len();
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
            adjacency_list: adjacency_list,
            particle_states: vec![0.0; num_nodes],
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

    // In your EntangledSystem's `impl` block

    /// Modifies the graph's topology based on the energy in particle_states.
    /// This simulates "curvature".
    /// Modifies the graph's topology based on the energy in particle_states.
    // Replace your existing update_curvature function with this refined version.
    pub fn update_curvature(&mut self, threshold: f64, max_distance: u32) {
        let active_nodes: Vec<usize> = self
            .particle_states
            .iter()
            .enumerate()
            .filter(|&(_, &state)| state > threshold)
            .map(|(index, _)| index)
            .collect();

        if active_nodes.is_empty() {
            println!(
                "[CURVATURE] No nodes exceeded the energy threshold of {}.",
                threshold
            );
            return;
        }

        let mut new_connections = Vec::new();

        // For each highly active node...
        for &active_node in &active_nodes {
            // ...find all nodes within its attraction range.
            let distances = self.bfs_distances(active_node);

            for (nearby_node, dist_option) in distances.iter().enumerate() {
                if let Some(dist) = *dist_option {
                    // The condition for attraction:
                    // 1. The node is within max_distance.
                    // 2. It is NOT a direct neighbor (distance > 1).
                    // 3. It is NOT the node itself (distance > 0).
                    if dist > 1 && dist <= max_distance {
                        println!(
                            "[CURVATURE] Active Node {} is attracting nearby Node {} (Distance: {})",
                            active_node, nearby_node, dist
                        );
                        // To avoid duplicates, we'll add the connection from smaller index to larger
                        new_connections
                            .push((active_node.min(nearby_node), active_node.max(nearby_node)));
                    }
                }
            }
        }

        // Remove duplicate connection pairs before adding them
        new_connections.sort();
        new_connections.dedup();

        // Add the new connections to the graph's adjacency list.
        for (node_a, node_b) in new_connections {
            self.adjacency_list[node_a].push(node_b);
            self.adjacency_list[node_b].push(node_a);
            println!(
                "[CURVATURE] New path created between Node {} and Node {}.",
                node_a, node_b
            );
        }
    }

    /// Applies the combined interference pattern from multiple pulse sources.
    pub fn apply_pulses(&mut self, sources: &[PulseSource]) {
        const WAVE_NUMBER: f64 = 1.0;
        let distance_maps: Vec<Vec<Option<u32>>> = sources
            .iter()
            .map(|s| self.bfs_distances(s.location_node_index))
            .collect();

        for node_index in 0..self.particle_states.len() {
            let mut total_effect = 0.0;
            for (source, distances) in sources.iter().zip(&distance_maps) {
                if let Some(Some(dist)) = distances.get(node_index) {
                    let distance = *dist as f64;
                    let phase = source.phase_offset - distance * WAVE_NUMBER;
                    let effect = source.amplitude * phase.cos();
                    total_effect += effect;
                }
            }
            self.particle_states[node_index] += total_effect;
        }
    }

    /// Calculates the shortest path distances from a start node to all other nodes in a graph.
    ///
    /// # Arguments
    /// * `start_node` - The index of the node to start the search from.
    /// * `adjacency_list` - A reference to the graph's structure.
    ///
    /// # Returns
    /// A vector where each index corresponds to a node and the value is the
    /// distance from the start_node. `None` if the node is unreachable.
    /// Calculates shortest path distances from a start node (BFS).
    /// Calculates shortest path distances from a start node (BFS).
    pub fn bfs_distances(&self, start_node: usize) -> Vec<Option<u32>> {
        let num_nodes = self.adjacency_list.len();
        let mut distances: Vec<Option<u32>> = vec![None; num_nodes];
        let mut queue: VecDeque<usize> = VecDeque::new();

        if start_node >= num_nodes {
            return distances;
        }

        distances[start_node] = Some(0);
        queue.push_back(start_node);

        while let Some(current_node) = queue.pop_front() {
            let current_dist = distances[current_node].unwrap();
            for &neighbor in &self.adjacency_list[current_node] {
                if distances[neighbor].is_none() {
                    distances[neighbor] = Some(current_dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }
        distances
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
    #[allow(unused_variables)]
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
                correlation_strength: 1.0,
                //                error_distance: 1.0,
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
        //   let phase_error = (average_state_b.arg() - expected_state_b.arg()).abs();

        FeedbackSignal {
            correlation_strength: correlation as f64,
        }
    }
}
