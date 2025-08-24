//! A semantically-typed, directed graph optimized for modification and traversal.
//!
//! This graph uses an adjacency list representation (`HashMap<NodeId, Vec<FractalGraphEdge>>`)
//! for efficient access to a node's outgoing edges. It is designed to be the
//! foundational structure for complex simulations.

use num_complex::Complex;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

// --- Core Data Structures ---

/// A unique identifier for a node in the graph.
/// Using a struct wrapper provides type safety over a raw u64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// Represents a directed connection between two nodes.
/// This is the core data-carrying element for your simulation.
#[derive(Clone, Debug, PartialEq)]
pub struct FractalGraphEdge {
    /// The destination node of this edge.
    pub destination: NodeId,
    /// The semantic type of the connection.
    pub edge_type: EdgeType,
    /// The complex-valued weight, representing the state (amplitude and phase).
    pub weight: Complex<f32>,
    // You could add other geometric data here later if needed,
    // like `length` or `direction`, but we'll keep it clean for now.
}

/// A generic container for the data stored within each node.
/// The `T` can be whatever you need for your simulation.
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: NodeId,
    pub payload: T,
}

/// Semantic classification of an edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeType {
    Excitatory,
    Inhibitory,
    Resonant,
    // Add other types relevant to your simulation
}

// --- Graph Definition ---

/// A directed graph using an adjacency list for its structure.
/// The generic type `T` represents the data stored in each node's payload.
#[derive(Debug)]
pub struct FractalGraph<T> {
    nodes: HashMap<NodeId, Node<T>>,
    /// Adjacency list: maps a source NodeId to a list of its outgoing edges.
    edges: HashMap<NodeId, Vec<FractalGraphEdge>>,
    next_node_id: u64,
}

// --- Error Types ---

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("Node not found: {0:?}")]
    NodeNotFound(NodeId),
    #[error("Edge already exists from {0:?} to {1:?} with type {2:?}")]
    DuplicateEdge(NodeId, NodeId, EdgeType),
}

// --- Implementation ---

// This impl block is for methods that work with ANY generic type T.
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

// The Default implementation can also be unbounded now.
impl<T> Default for FractalGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}

// This impl block is for methods that REQUIRE T to have certain traits.
impl<T: Debug + PartialEq> FractalGraph<T> {
    /// Adds a new node to the graph with the given payload.
    /// Returns the `NodeId` of the newly created node.
    pub fn add_node(&mut self, payload: T) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = Node { id, payload };
        self.nodes.insert(id, node);
        // Also create an empty edge list for the new node.
        self.edges.insert(id, Vec::new());

        id
    }

    /// Adds a directed edge between two existing nodes.
    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge_type: EdgeType,
        initial_weight: Complex<f32>,
    ) -> Result<(), GraphError> {
        // Ensure both nodes exist.
        if !self.nodes.contains_key(&from) {
            return Err(GraphError::NodeNotFound(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(GraphError::NodeNotFound(to));
        }

        // Get the list of outgoing edges for the 'from' node.
        let outgoing_edges = self.edges.get_mut(&from).unwrap(); // Safe due to check above and `add_node` logic

        // Check for duplicate edges.
        if outgoing_edges
            .iter()
            .any(|edge| edge.destination == to && edge.edge_type == edge_type)
        {
            return Err(GraphError::DuplicateEdge(from, to, edge_type));
        }

        // Add the new edge.
        outgoing_edges.push(FractalGraphEdge {
            destination: to,
            edge_type,
            weight: initial_weight,
        });

        Ok(())
    }

    /// Removes a node and all edges connected to it.
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<Node<T>, GraphError> {
        // Remove the node itself and its outgoing edges.
        let removed_node = self
            .nodes
            .remove(&node_id)
            .ok_or(GraphError::NodeNotFound(node_id))?;
        self.edges.remove(&node_id);

        // Remove all incoming edges that pointed to the deleted node.
        for (_id, outgoing_edges) in self.edges.iter_mut() {
            outgoing_edges.retain(|edge| edge.destination != node_id);
        }

        Ok(removed_node)
    }

    /// Gets a mutable reference to the list of edges originating from a node.
    /// This is the key function for modifying particle states.
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

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }

    /// Checks if the graph contains any cycles. Useful for certain algorithms.
    pub fn is_acyclic(&self) -> bool {
        let mut visited = HashMap::new();
        let mut recursion_stack = HashMap::new();

        for node_id in self.nodes.keys() {
            if self._is_cyclic_util(*node_id, &mut visited, &mut recursion_stack) {
                return false;
            }
        }
        true
    }

    // Helper for is_acyclic
    fn _is_cyclic_util(
        &self,
        node_id: NodeId,
        visited: &mut HashMap<NodeId, bool>,
        recursion_stack: &mut HashMap<NodeId, bool>,
    ) -> bool {
        if recursion_stack.get(&node_id).cloned().unwrap_or(false) {
            return true; // Cycle detected
        }
        if visited.get(&node_id).cloned().unwrap_or(false) {
            return false; // Already visited, no cycle found from here
        }

        visited.insert(node_id, true);
        recursion_stack.insert(node_id, true);

        if let Some(outgoing_edges) = self.edges.get(&node_id) {
            for edge in outgoing_edges {
                if self._is_cyclic_util(edge.destination, visited, recursion_stack) {
                    return true;
                }
            }
        }

        recursion_stack.insert(node_id, false);
        false
    }

    /// Returns an iterator that allows modifying each edge in the graph.
    pub fn all_edges_mut(&mut self) -> impl Iterator<Item = &mut FractalGraphEdge> {
        self.edges.values_mut().flatten()
    }
}
