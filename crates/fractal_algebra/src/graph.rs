use std::collections::HashMap;
use factorial_engine::FactorialEngine;

// A more descriptive name for your vecmap structure.
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

/// A custom graph structure to store and analyze the relationships
/// between factorial factorizations.
pub struct FractalGraph {
    // Primary storage: maps a number n to its full Node object.
    nodes: HashMap<u64, FactorialNode>,

    // --- Secondary Indices for Fast Queries ---

    // An "inverted index" mapping a prime to all nodes (n) that contain it.
    // This is part of the "Trie-like" functionality.
    prime_to_n_index: HashMap<u64, Vec<u64>>,
}

impl FractalGraph {
    pub fn new() -> Self {
        FractalGraph {
            nodes: HashMap::new(),
            prime_to_n_index: HashMap::new(),
        }
    }

    /// Adds a node for n! to the graph.
    /// `factorization_engine` is the tool we built yesterday.
    pub fn add_node(&mut self, n: u64, engine: &mut FactorialEngine) {
        let factorization_hashmap = engine.get_factorial_factorization(n);
        
        // Convert HashMap to a sorted Vec for a canonical representation.
        let mut factorization_map: FactorizationMap = factorization_hashmap.into_iter().collect();
        factorization_map.sort_by_key(|&(p, _)| p);

        // Update the prime_to_n_index
        for &(prime, _) in &factorization_map {
            self.prime_to_n_index
                .entry(prime)
                .or_default()
                .push(n);
        }

        let node = FactorialNode {
            n,
            factorization: factorization_map,
            category: 0, // Placeholder category
        };
        self.nodes.insert(n, node);
    }

    pub fn get_node(&self, n: u64) -> Option<&FactorialNode> {
        self.nodes.get(&n)
    }
    
    /// A "dynamic relationship" query: find nodes that are similar to a given node.
    /// This is where we make the graph relationships real.
    pub fn find_similar_nodes(&self, n: u64, top_k: usize) -> Vec<&FactorialNode> {
        let target_node = match self.get_node(n) {
            Some(node) => node,
            None => return Vec::new(),
        };

        let mut similarities = Vec::new();
        for other_node in self.nodes.values() {
            if other_node.n == n { continue; }
            
            let score = Self::calculate_cosine_similarity(
                &target_node.factorization,
                &other_node.factorization
            );
            similarities.push((score, other_node));
        }

        similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        similarities.into_iter().take(top_k).map(|(_, node)| node).collect()
    }

    /// Helper function to compute similarity between two factorizations.
    /// This is a concrete implementation of "shared traits".
    fn calculate_cosine_similarity(f1: &FactorizationMap, f2: &FactorizationMap) -> f64 {
        // This function would convert the two factorization maps into vectors
        // in a common prime basis, and then compute their cosine similarity.
        // (A dot product divided by the product of their magnitudes).
        // A score of 1.0 means identical proportions of prime factors.
        // A score of 0.0 means no shared prime factors.
        
        // Implementation logic would go here...
        0.0 // Placeholder
    }
    
    /// A "fractal traversal" query: zoom in on a node's prime components.
    pub fn get_prime_factor_nodes(&self, n: u64) -> Vec<&FactorialNode> {
        match self.get_node(n) {
            Some(node) => node.factorization.iter()
                .filter_map(|&(prime, _)| self.get_node(prime))
                .collect(),
            None => Vec::new(),
        }
    }
}

