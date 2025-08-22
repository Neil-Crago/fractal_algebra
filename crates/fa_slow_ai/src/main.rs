//! The main binary for the Slow AI experiment.
//! This crate orchestrates the learning loop, combining the Bayesian search
//! with the quantum simulation defined in the EntangledSystem.

mod entangled_system;

// Use shared types from the core API crate
use fa_core::state::{EntropyPulse, FeedbackSignal};
use fa_core::traits::ProbabilisticSearch;

// Use the concrete implementation of the learning engine
use fa_bayes::FrequencyBeliefSpace;

// Use the simulation engine
use entangled_system::EntangledSystem;

/// This is the new, real experiment function.
/// It creates a quantum system, applies the AI's pulse, and measures the result.
fn run_real_experiment(pulse: &EntropyPulse) -> FeedbackSignal {
    // 1. Create a fresh, perfectly entangled system for each experiment.
    //    This ensures each test is independent.
    let mut system = EntangledSystem::new(10, 10); // A 10x10 grid universe

    // 2. Apply the AI's proposed pulse, which will cause some decoherence.
    system.apply_pulse(pulse);

    // 3. Perform the measurement and return the resulting correlation.
    system.measure_correlation()
}


// The AI's learning loop is implemented in the `main` function below.
// It sets up the belief space, runs multiple iterations of guessing,
// and updates its beliefs based on feedback from the real experiment.
fn main() {
    println!("\n--- Slow AI Entanglement Recoherence Experiment ---");

    for _k in 1..=25 {
        // 1. Setup the Bayesian learning agent.
        //    We'll start with a wide search range for the frequency.
        let mut belief_space = FrequencyBeliefSpace::new(0.0, 1.0); // Start guess at 0 Hz, amplitude 1.0

        // We need to define what the "correct" frequency is for our simulation.
        // In a real experiment, this would be unknown. Here, we define it so we can
        // see if the AI can discover it. Let's choose a non-obvious number.
        const TARGET_RESONANT_FREQUENCY: f64 = std::f64::consts::PI / 2.0; // approx 1.57

        // 2. The main learning loop.
        for _i in 1..=10_000 {
            // Run for 100 iterations to give the AI time to learn
            // a. The AI proposes its best guess based on its current beliefs.
            let guess_pulse = belief_space.propose_best_guess();

            // b. We run the *real* physics simulation with this guess.
            //    To make the simulation work, we need to compare the AI's guess
            //    to our hidden "resonant frequency". A pulse at the resonant frequency
            //    should cause minimal decoherence.
            //    We simulate this by adjusting the pulse's frequency relative to the target.
            //    A perfect guess (guess_pulse.frequency == TARGET_RESONANT_FREQUENCY)
            //    results in a frequency of 0, causing no phase rotation.
            let effective_pulse = EntropyPulse {
                frequency: guess_pulse.frequency - TARGET_RESONANT_FREQUENCY,
                amplitude: guess_pulse.amplitude,
                waveform: guess_pulse.waveform.clone(),
            };

            let feedback = run_real_experiment(&effective_pulse);

            // c. The AI updates its beliefs based on the feedback from the real experiment.
            belief_space.update(&feedback, &guess_pulse);
        }

        //println!("\n--- Learning Complete ---");
        let final_guess = belief_space.propose_best_guess();
        let metric_fract = (final_guess.frequency / (2.0 * std::f64::consts::PI))
            .fract()
            .abs();
        let mut category = "Saddle Point (Stable Decoherence)"; // Default to saddle point

        // Check if the fractional part is closer to 0.25 than to 0.75
        // We can check if it's within a certain tolerance of the peak.
        if (metric_fract - 0.25).abs() < 0.1 {
            category = "Peak (Coherent Resonance)";
        }

        println!(
            "guess: {:>.4} rad/s,   \tmetric = {:>.4},  \tcategory = {}",
            final_guess.frequency,
            metric_fract, // It's clearer to print the fractional part
            category
        );
    }
}
