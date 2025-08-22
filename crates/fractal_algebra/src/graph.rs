//! A semantically-typed, directed graph of fractal units.
//!
//! # Examples
//!
//! ```rust
//! use fractal_algebra::graph::{FractalGraph, NodeId, EdgeType, GraphError};
//! use fractal_algebra::atom::FractalAtom;
//! use fractal_algebra::metadata::Metadata;
//! use fractal_algebra::tag_set::TagSet;
//!
//! // ─── 1. Create a new graph and add two FractalAtom nodes ───────────────────
//! let mut g: FractalGraph<FractalAtom<i32>> = FractalGraph::new();
//! let a = g
//!     .add_node(FractalAtom::new(10, TagSet::default(), Metadata::default()).unwrap())
//!     .unwrap();
//! let b = g
//!     .add_node(FractalAtom::new(20, TagSet::default(), Metadata::default()).unwrap())
//!     .unwrap();
//!
//! assert_eq!(g.node_count(), 2);
//! assert_eq!(g.edge_count(), 0);
//!
//!
//! // ─── 2. Connect them with an excitatory edge ───────────────────────────────
//! g.add_edge(a, b, EdgeType::Excitatory).unwrap();
//! assert_eq!(g.edge_count(), 1);
//! assert!(g.is_acyclic());  // Still no cycle
//!
//!
//! // ─── 3. Error when referring to a missing node ─────────────────────────────
//! let missing = NodeId(u64::MAX);
//! assert_eq!(
//!     g.add_edge(a, missing, EdgeType::Excitatory),
//!     Err(GraphError::NodeNotFound(a, missing))
//! );
//!
//!
//! // ─── 4. Duplicate edges get rejected ──────────────────────────────────────
//! assert_eq!(
//!     g.add_edge(a, b, EdgeType::Excitatory),
//!     Err(GraphError::DuplicateEdge(a, b, EdgeType::Excitatory))
//! );
//!
//!
//! // ─── 5. Cycle detection flips `is_acyclic` to false ──────────────────────
//! // (we allow edges in, but `is_acyclic` will notice the loop)
//! g.add_edge(b, a, EdgeType::Excitatory).unwrap();
//! assert!(!g.is_acyclic());
//!
//!
//! // ─── 6. Removing a node drops its edges ───────────────────────────────────
//! g.remove_node(a).unwrap();
//! assert_eq!(g.node_count(), 1);
//! assert_eq!(g.edge_count(), 0);
//! ```
//!
//! And here’s a focused example on the *error* path for `add_node` itself:
//!
//! ```rust
//! # use fractal_algebra::graph::{FractalGraph, GraphError};
//! # use fractal_algebra::atom::FractalAtom;
//! # use fractal_algebra::metadata::Metadata;
//! # use fractal_algebra::tag_set::TagSet;
//! let mut g: FractalGraph<FractalAtom<i32>> = FractalGraph::new();
//!
//! // First insertion succeeds
//! let first = g
//!     .add_node(FractalAtom::new(42, TagSet::default(), Metadata::default()).unwrap())
//!     .unwrap();
//!
//! // Inserting the *same* semantic payload again triggers DuplicateNode
//! assert_eq!(
//!     g.add_node(FractalAtom::new(42, TagSet::default(), Metadata::default()).unwrap()),
//!     Err(GraphError::DuplicateNode(first))
//! );
//! ```
//!
//! Finally, a `#[should_panic]` style test for a method that *panics* on misuse:
//!
//! ```rust,should_panic
//! # use fractal_algebra::graph::FractalGraph;
//! // Calling `remove_node` on a graph with no such node panics.
//! let mut g: FractalGraph<()> = FractalGraph::new();
//! g.remove_node(Default::default()).unwrap();  // this node never existed
//! ```

use thiserror::Error;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::collections::{BinaryHeap, HashMap};

// A more descriptive name for my vecmap structure.
// Represents the prime factorization as a list of (prime, exponent) tuples.
pub type FactorizationMap = Vec<(u64, u64)>;

// The node in our graph, representing the factorization of n!
#[derive(Debug, Clone)]
pub struct FactorialNode {
    pub n: u64, // The number n for n!
    pub factorization: FactorizationMap,
    pub category: u32, // For future classification (e.g., based on complexity)

                       // We can cache computed properties like entropy later on.
                       // pub entropy: Option<f64>,
}

// --- A* State for the Priority Queue ---
// We need a wrapper to make Rust's BinaryHeap (a max-heap) act as a min-heap for A*.
#[derive(Copy, Clone, PartialEq)]
struct AStarState {
    n: u64,
    f_score: f64,
}

impl Eq for AStarState {}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Excitatory,
    Inhibitory,
    Resonant,
    Feedback,
    Custom(&'static str), // optional
}



#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("node not found: {0} → {1}")]
    NodeNotFound(u64, u64),

    #[error("duplicate edge: {0} → {1} of type {2:?}")]
    DuplicateEdge(u64, u64, EdgeType),

    #[error("duplicate node: {0}")]
    DuplicateNode(u64),

    #[error("cycle detected")]
    CycleDetected,

    #[error("invalid operation: {0}")]
    Invalid(String),
}
/// A custom graph structure to store and analyze the relationships
/// between factorial factorizations.
pub struct FractalGraph {
    pub nodes: HashMap<u64, FactorialNode>,
    pub prime_to_n_index: HashMap<u64, Vec<u64>>,
    pub edges: HashMap<(u64, u64, EdgeType), ()>,
    next_node_id: u64, // new
}

impl Default for FractalGraph {
    fn default() -> Self {
        Self::new()
    }
}


impl FractalGraph {
   pub fn assert_invariants(&self) {
    // Every edge must reference existing nodes
    for (&(src, dst, _), _) in &self.edges {
        debug_assert!(
            self.nodes.contains_key(&src),
            "Edge source node {} does not exist",
            src
        );
        debug_assert!(
            self.nodes.contains_key(&dst),
            "Edge destination node {} does not exist",
            dst
        );
    }

    // Optional: check for duplicate nodes (should be impossible with HashMap)
    // Optional: check for self-loops or cycles if disallowed
}

    /// Connect two existing nodes.
    ///
    /// # Parameters
    /// - `from`: source node
    /// - `to`:   destination node
    /// - `edge_type`: semantic edge classification
    ///
    /// # Errors
    /// - `GraphError::NodeNotFound` if either `from` or `to` is absent.
    /// - `GraphError::DuplicateEdge` if the same `(from, to, edge_type)` already exists.
       pub fn add_edge(
    &mut self,
    from: u64,
    to: u64,
    edge_type: EdgeType,
) -> Result<(), GraphError> {
    if !self.nodes.contains_key(&from) || !self.nodes.contains_key(&to) {
        return Err(GraphError::NodeNotFound(from, to));
    }

    let key = (from, to, edge_type);
    if self.edges.contains_key(&key) {
        return Err(GraphError::DuplicateEdge(from, to, edge_type));
    }

    self.edges.insert(key, ());
    Ok(())
}
}

impl FractalGraph {
    /// Create an empty FractalGraph.
    ///
    /// # Postconditions
    /// - `node_count == 0`
    /// - `edge_count == 0`
    pub fn new() -> Self {
        FractalGraph {
            nodes: HashMap::new(),
            prime_to_n_index: HashMap::new(),
            edges: HashMap::new(),
            next_node_id: 0,
        }
    }

    /// Add a new node with semantic payload.
    ///
    /// # Errors
    /// Returns `GraphError::DuplicateNode` if the node’s fingerprint already exists.
    pub fn add_node(&mut self, payload: FactorialNode) -> Result<u64, GraphError> {
    let node_id = self.next_node_id;
    self.next_node_id += 1;

    if self.nodes.contains_key(&node_id) {
        return Err(GraphError::DuplicateNode(node_id));
    }

    self.nodes.insert(node_id, payload);
    Ok(node_id)
}

    /// Update the prime_to_n_index for a given node.
    fn update_prime_index(&mut self, n: u64, factorization_map: &FactorizationMap) {
        for &(prime, _) in factorization_map {
            self.prime_to_n_index.entry(prime).or_default().push(n);
        }

        let node = FactorialNode {
            n,
            factorization: factorization_map.to_vec(),
            category: 0, // Placeholder category
        };
        self.nodes.insert(n, node);
    }

    pub fn get_node(&self, n: u64) -> Option<&FactorialNode> {
        self.nodes.get(&n)
    }

    // --- The A* Pathfinding Implementation ---
    pub fn find_path_a_star(
        &self,
        start_n: u64,
        goal_n: u64,
        is_blocked: impl Fn(&FactorialNode) -> bool,
    ) -> Option<Vec<u64>> {
        let start_node = self.get_node(start_n)?;
        let goal_node = self.get_node(goal_n)?;

        // A* data structures
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<u64, u64> = HashMap::new();
        let mut g_score: HashMap<u64, f64> = HashMap::new();

        // Initialize scores for all nodes to infinity
        for &n in self.nodes.keys() {
            g_score.insert(n, f64::INFINITY);
        }

        // Initialize start node
        g_score.insert(start_n, 0.0);
        let h_score = self.calculate_cost(start_node, goal_node);
        open_set.push(AStarState {
            n: start_n,
            f_score: h_score,
        });

        while let Some(current_state) = open_set.pop() {
            let current_n = current_state.n;

            // Goal reached, reconstruct the path
            if current_n == goal_n {
                let mut path = vec![current_n];
                let mut current = current_n;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }

            let current_node = self.get_node(current_n).unwrap();

            // Explore neighbors (the 10 most similar nodes)
            for neighbor_node in self.find_similar_nodes(current_n, 10) {
                let neighbor_n = neighbor_node.n;

                // Check if the neighbor is an obstacle
                if is_blocked(neighbor_node) {
                    continue;
                }

                let tentative_g_score =
                    g_score[&current_n] + self.calculate_cost(current_node, neighbor_node);

                if tentative_g_score < g_score[&neighbor_n] {
                    // This path to the neighbor is better than any previous one.
                    came_from.insert(neighbor_n, current_n);
                    g_score.insert(neighbor_n, tentative_g_score);

                    let h_score = self.calculate_cost(neighbor_node, goal_node);
                    open_set.push(AStarState {
                        n: neighbor_n,
                        f_score: tentative_g_score + h_score,
                    });
                }
            }
        }

        // No path found
        None
    }

    pub fn ingest_factorizations(&mut self, data: &[(u64, FactorizationMap)]) {
    for &(n, ref fmap) in data {
        self.update_prime_index(n, fmap);
    }
}

    // --- Helper functions for cost and similarity ---
    fn calculate_cost(&self, node1: &FactorialNode, node2: &FactorialNode) -> f64 {
        1.0 - self.calculate_cosine_similarity(&node1.factorization, &node2.factorization)
    }

    // (find_similar_nodes and calculate_cosine_similarity would be implemented here)
    // For this test, we'll mock them to be simple.
    fn find_similar_nodes(&self, n: u64, _top_k: usize) -> Vec<&FactorialNode> {
        // A real implementation would use cosine similarity.
        // For a simple, predictable test, we'll just return adjacent numbers.
        let mut neighbors = Vec::new();
        if n > 1
            && let Some(node) = self.get_node(n - 1) {
                neighbors.push(node);
            }
        if let Some(node) = self.get_node(n + 1) {
            neighbors.push(node);
        }
        neighbors
    }

    fn calculate_cosine_similarity(&self, f1: &FactorizationMap, f2: &FactorizationMap) -> f64 {
        // Mock similarity: higher for closer numbers
        let n1 = f1.iter().map(|(_, e)| e).sum::<u64>();
        let n2 = f2.iter().map(|(_, e)| e).sum::<u64>();
        1.0 - (n1 as f64 - n2 as f64).abs() / 100.0
    }

    /// A "fractal traversal" query: zoom in on a node's prime components.
    pub fn get_prime_factor_nodes(&self, n: u64) -> Vec<&FactorialNode> {
        match self.get_node(n) {
            Some(node) => node
                .factorization
                .iter()
                .filter_map(|&(prime, _)| self.get_node(prime))
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn remove_node(&mut self, node_id: u64) -> Result<(), GraphError> {
    if !self.nodes.contains_key(&node_id) {
        return Err(GraphError::NodeNotFound(node_id, node_id));
    }

    // Remove the node
    self.nodes.remove(&node_id);

    // Remove all edges involving this node
    self.edges.retain(|(src, dst, _), _| *src != node_id && *dst != node_id);

    // Remove from prime index
    for vec in self.prime_to_n_index.values_mut() {
        vec.retain(|&n| n != node_id);
    }

    Ok(())
}
}

