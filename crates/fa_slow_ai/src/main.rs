// Use the library components from this crate
use fa_core::state::{EntropyPulse, FeedbackSignal};
use fa_core::traits::ProbabilisticSearch;

// Use the concrete implementation from your other crate
use fa_bayes::FrequencyBeliefSpace;

// A dummy function to simulate the physics experiment for now
fn run_experiment(pulse: &EntropyPulse) -> FeedbackSignal {
    const TARGET_FREQUENCY: f64 = 42.0;
    let distance_from_target = (pulse.frequency - TARGET_FREQUENCY).abs();

    // Make the decay much gentler by dividing by a larger number
    let correlation_strength = (-(distance_from_target / 50.0)).exp(); // Use 50.0 instead of 10.0

    FeedbackSignal {
        correlation_strength,
        phase_alignment_error: distance_from_target,
    }
}

fn main() {
    println!("--- Slow AI Entanglement Experiment ---");

    // 1. Setup the Bayesian learning agent
    let mut belief_space = FrequencyBeliefSpace::new(100.0, 1.0); // Start with an initial, incorrect guess

    // 2. The main learning loop
    for i in 1..=100 {
        // a. The AI proposes its best guess
        let guess_pulse = belief_space.propose_best_guess();
        println!(
            "Loop {}: Proposing frequency {:.2} Hz...",
            i, guess_pulse.frequency
        );

        // b. We run our (dummy) experiment with this guess
        let feedback = run_experiment(&guess_pulse);
        println!(
            "  -> Feedback received. Correlation: {:.3}",
            feedback.correlation_strength
        );

        // c. The AI updates its beliefs based on the feedback
        belief_space.update(&feedback, &guess_pulse);
    }

    println!("\n--- Learning Complete ---");
    let final_guess = belief_space.propose_best_guess();
    println!(
        "Final belief for correct frequency: {:.2} Hz",
        final_guess.frequency
    );
}
