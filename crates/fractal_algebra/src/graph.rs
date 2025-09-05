//! A semantically-typed, directed graph optimized for modification and traversal.
//!
//! This module provides `FractalGraph`, a core data structure for the Fractal_Algebra project.
//! It uses an adjacency list representation (`HashMap<NodeId, Vec<FractalGraphEdge>>`)
//! for efficient O(1) average-case access to a node's outgoing edges. It is designed to be the
//! foundational structure for complex, quantum-inspired simulations where the state is stored
//! on the edges as complex numbers.

use num_complex::Complex;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

// --- Core Data Structures ---

/// A unique identifier for a node in the graph.
///
/// Using a struct wrapper around a raw integer provides strong type safety, preventing
/// accidental use of other integer types as a node ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Represents a directed connection between two nodes, carrying the simulation state.
#[derive(Clone, Debug, PartialEq)]
pub struct FractalGraphEdge {
    /// The destination node of this edge.
    pub destination: NodeId,
    /// The semantic type of the connection (e.g., excitatory, inhibitory).
    pub edge_type: EdgeType,
    /// The complex-valued weight, representing the state (e.g., a quantum amplitude and phase).
    pub weight: Complex<f32>,
}

/// A generic container for the data stored within each node.
///
/// The payload `T` can be any data type relevant to the simulation's nodes.
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: NodeId,
    pub payload: T,
}

/// A semantic classification for an edge, defining its functional role in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Excitatory,
    Inhibitory,
    Resonant,
}

// --- Graph Definition ---

/// A directed graph using an adjacency list representation.
///
/// The generic type `T` represents the data stored in each node's payload.
#[derive(Debug)]
pub struct FractalGraph<T> {
    /// Stores the nodes of the graph, mapped by their unique `NodeId`.
    nodes: HashMap<NodeId, Node<T>>,
    /// The adjacency list: maps a source `NodeId` to a vector of its outgoing edges.
    edges: HashMap<NodeId, Vec<FractalGraphEdge>>,
    /// An internal counter to ensure newly created nodes have a unique ID.
    next_node_id: u64,
}

// --- Error Types ---

/// Defines errors that can occur during graph manipulation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("Node not found: {0:?}")]
    NodeNotFound(NodeId),
    #[error("Edge already exists from {0:?} to {1:?} with type {2:?}")]
    DuplicateEdge(NodeId, NodeId, EdgeType),
}

// --- Implementation ---

impl<T> FractalGraph<T> {
    /// Creates a new, empty `FractalGraph`.
    pub fn new() -> Self {
        FractalGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_node_id: 0,
        }
    }
}

impl<T> Default for FractalGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}

// This block contains methods that require the node payload `T` to support debugging and comparison.
impl<T: Debug + PartialEq> FractalGraph<T> {
    /// Adds a new node to the graph with the given payload.
    ///
    /// # Returns
    /// The `NodeId` of the newly created node.
    pub fn add_node(&mut self, payload: T) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = Node { id, payload };
        self.nodes.insert(id, node);
        // Ensure every node has an entry in the edges map, even if it has no outgoing edges.
        self.edges.insert(id, Vec::new());

        id
    }

    /// Adds a directed edge between two existing nodes.
    ///
    /// # Errors
    /// Returns `GraphError::NodeNotFound` if `from` or `to` IDs are invalid.
    /// Returns `GraphError::DuplicateEdge` if an edge of the same type already exists
    /// between the two nodes.
    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge_type: EdgeType,
        initial_weight: Complex<f32>,
    ) -> Result<(), GraphError> {
        // Ensure both the source and destination nodes exist.
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::NodeNotFound(to));
        }

        // Get the list of outgoing edges for the 'from' node.
        // This unwrap is safe because we create an edge entry for every node in `add_node`.
        let outgoing_edges = self.edges.get_mut(&from).unwrap();

        // Prevent adding a duplicate edge (same source, dest, and type).
        if outgoing_edges
            .iter()
            .any(|edge| edge.destination == to && edge.edge_type == edge_type)
        {
            return Err(GraphError::DuplicateEdge(from, to, edge_type));
        }

        outgoing_edges.push(FractalGraphEdge {
            destination: to,
            edge_type,
            weight: initial_weight,
        });

        Ok(())
    }

    /// Removes a node and all edges connected to it (both incoming and outgoing).
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<Node<T>, GraphError> {
        // First, remove the node from the nodes map. This also removes its outgoing edges list.
        let removed_node = self.nodes.remove(&node_id).ok_or(GraphError::NodeNotFound(node_id))?;
        self.edges.remove(&node_id);

        // Then, iterate through all other nodes and remove any incoming edges pointing to the deleted node.
        for (_id, outgoing_edges) in self.edges.iter_mut() {
            outgoing_edges.retain(|edge| edge.destination != node_id);
        }

        Ok(removed_node)
    }

    /// Gets a mutable reference to the list of edges originating from a node.
    /// This is a key function for efficiently modifying the state of connections
    /// in the simulation (e.g., updating a particle's resonance pattern).
    pub fn get_edges_for_node_mut(
        &mut self,
        node_id: NodeId,
    ) -> Option<&mut Vec<FractalGraphEdge>> {
        self.edges.get_mut(&node_id)
    }

    /// Gets an immutable reference to a node's payload.
    pub fn get_node(&self, node_id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(&node_id)
    }

    // --- Utility Functions ---

    /// Returns the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the total number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }

    /// Checks if the graph is a Directed Acyclic Graph (DAG).
    /// This is useful for algorithms that require no cycles, such as topological sorting.
    pub fn is_acyclic(&self) -> bool {
        let mut visited = HashMap::new(); // Nodes we've already processed.
        let mut recursion_stack = HashMap::new(); // Nodes in the current traversal path.

        for node_id in self.nodes.keys() {
            if self._is_cyclic_util(*node_id, &mut visited, &mut recursion_stack) {
                return false; // A cycle was detected.
            }
        }
        true // No cycles found after checking all nodes.
    }

    /// A recursive helper function for `is_acyclic` that performs a depth-first search.
    fn _is_cyclic_util(
        &self,
        node_id: NodeId,
        visited: &mut HashMap<NodeId, bool>,
        recursion_stack: &mut HashMap<NodeId, bool>,
    ) -> bool {
        // If the node is in the current recursion stack, we've found a back edge -> cycle.
        if recursion_stack.get(&node_id).cloned().unwrap_or(false) {
            return true;
        }
        // If the node has been visited in a previous traversal, we know there's no cycle from it.
        if visited.get(&node_id).cloned().unwrap_or(false) {
            return false;
        }

        // Mark the current node as visited and add it to the recursion stack.
        visited.insert(node_id, true);
        recursion_stack.insert(node_id, true);

        // Recur for all neighbors (destinations of outgoing edges).
        if let Some(outgoing_edges) = self.edges.get(&node_id) {
            for edge in outgoing_edges {
                if self._is_cyclic_util(edge.destination, visited, recursion_stack) {
                    return true;
                }
            }
        }

        // Remove the node from the recursion stack as we backtrack.
        recursion_stack.insert(node_id, false);
        false
    }

    /// Returns an iterator that allows modifying each edge in the entire graph.
    pub fn all_edges_mut(&mut self) -> impl Iterator<Item = &mut FractalGraphEdge> {
        self.edges.values_mut().flatten()
    }
}