use factorial_engine::FactorialEngine;
use fractal_algebra::graph::{FactorialNode, FractalGraph};

fn main() {
    println!("Setting up the A* test on FractalGraph...");
    let mut engine = FactorialEngine::new(Some(100));
    let mut graph = FractalGraph::new();

    // Populate the graph with nodes from n=1 to n=100
    for n in 1..=100 {
        graph.add_node(n, &mut engine);
    }
    println!("Graph populated with {} nodes.", graph.nodes.len());

    // Define the obstacle condition
    let is_blocked = |node: &FactorialNode| {
        for &(prime, exponent) in &node.factorization {
            if prime == 7 {
                return (3..=7).contains(&exponent);
            }
        }
        false
    };

    // The exponent of 7 is >= 3 from n=21 to n=48.
    println!("\nNodes where n is between 21 and 48 will be treated as obstacles.");

    let start_n = 15;
    let goal_n = 50;

    println!(
        "Attempting to find a path from n={} to n={}...",
        start_n, goal_n
    );

    match graph.find_path_a_star(start_n, goal_n, is_blocked) {
        Some(path) => {
            println!("\nPath found!");
            println!("{:?}", path);
            assert!(
                path.iter().all(|&n| !(21..=48).contains(&n)),
                "Path illegally entered a blocked region!"
            );
            println!(
                "\nVerification successful: The path correctly navigated around the blocked nodes."
            );
        }
        None => {
            println!("\nNo path found.");
        }
    }
}

// NOTE: I would need to add the full implementations for FactorialEngine,
// FractalGraph::new, add_node, etc. for this to be a complete, runnable example.
// The mocked `find_similar_nodes` is used here for simplicity
