use factorial_engine::FactorialEngine;
use std::cmp::Ordering;
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

/// A custom graph structure to store and analyze the relationships
/// between factorial factorizations.
pub struct FractalGraph {
    pub nodes: HashMap<u64, FactorialNode>,
    pub prime_to_n_index: HashMap<u64, Vec<u64>>,
}

impl Default for FractalGraph {
    fn default() -> Self {
        Self::new()
    }
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
            self.prime_to_n_index.entry(prime).or_default().push(n);
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
}
